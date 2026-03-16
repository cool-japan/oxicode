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

// ── Domain types: Water Treatment & Wastewater Management ────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlarmSeverity {
    Information,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FilterMediaType {
    Sand,
    ActivatedCarbon,
    Anthracite,
    Membrane,
    DualMedia,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ComplaintCategory {
    Taste,
    Odor,
    Discoloration,
    LowPressure,
    ServiceInterruption,
    BillingDispute,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NutrientRemovalProcess {
    Anoxic,
    Aerobic,
    Anaerobic,
    ModifiedBardenpho,
    SequencingBatchReactor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InfluentQualitySample {
    sample_id: u64,
    timestamp_epoch: u64,
    bod_mg_l: u32,
    cod_mg_l: u32,
    tss_mg_l: u32,
    ph_x100: u16,
    turbidity_ntu_x10: u32,
    ammonia_mg_l_x100: u32,
    flow_rate_m3_h: u32,
    temperature_c_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EffluentQualitySample {
    sample_id: u64,
    timestamp_epoch: u64,
    bod_mg_l: u32,
    cod_mg_l: u32,
    tss_mg_l: u32,
    ph_x100: u16,
    turbidity_ntu_x10: u32,
    total_nitrogen_mg_l_x100: u32,
    total_phosphorus_mg_l_x100: u32,
    fecal_coliform_cfu_100ml: u32,
    compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChemicalDosingRecord {
    record_id: u64,
    chemical_name: String,
    dosage_mg_l_x100: u32,
    flow_rate_gph: u32,
    injection_point: String,
    operator_id: String,
    timestamp_epoch: u64,
    tank_level_pct_x10: u16,
    batch_lot_number: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FiltrationSystemStatus {
    filter_id: u32,
    media_type: FilterMediaType,
    head_loss_m_x100: u32,
    flow_rate_m3_h_x10: u32,
    turbidity_out_ntu_x100: u32,
    run_hours_since_backwash: u32,
    backwash_count: u32,
    last_backwash_epoch: u64,
    online: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UvDisinfectionReading {
    unit_id: u32,
    timestamp_epoch: u64,
    dose_mj_cm2_x100: u32,
    transmittance_pct_x10: u16,
    lamp_hours: u32,
    lamp_intensity_pct_x10: u16,
    flow_rate_m3_h: u32,
    alarm_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MbrPerformance {
    train_id: u32,
    timestamp_epoch: u64,
    trans_membrane_pressure_kpa_x10: u32,
    permeate_flux_lmh_x100: u32,
    mlss_mg_l: u32,
    srt_days: u16,
    dissolved_oxygen_mg_l_x100: u16,
    permeate_turbidity_ntu_x1000: u32,
    chemical_clean_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SludgeProcessingParams {
    batch_id: u64,
    total_solids_pct_x100: u32,
    volatile_solids_pct_x100: u32,
    dewatered_cake_pct_x100: u32,
    polymer_dose_kg_dry_ton_x10: u32,
    digester_temp_c_x10: u16,
    retention_time_days: u16,
    biogas_m3_h_x10: u32,
    disposal_method: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScadaAlarmEvent {
    alarm_id: u64,
    timestamp_epoch: u64,
    severity: AlarmSeverity,
    source_tag: String,
    description: String,
    value_at_trigger: u32,
    setpoint: u32,
    acknowledged: bool,
    resolved_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegulatoryComplianceSample {
    sample_id: u64,
    permit_id: String,
    parameter_name: String,
    measured_value_x1000: u32,
    permit_limit_x1000: u32,
    unit: String,
    sampling_location: String,
    lab_id: String,
    compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DistributionNetworkReading {
    node_id: u32,
    timestamp_epoch: u64,
    pressure_psi_x100: u32,
    flow_gpm_x10: u32,
    chlorine_residual_mg_l_x100: u16,
    temperature_c_x10: u16,
    zone_id: u16,
    valve_status_open: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StormwaterOverflowEvent {
    event_id: u64,
    outfall_id: String,
    start_epoch: u64,
    end_epoch: u64,
    volume_gallons: u64,
    peak_flow_gpm: u32,
    rainfall_inches_x100: u32,
    receiving_water: String,
    reported_to_agency: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BiogasGenerationRecord {
    digester_id: u32,
    timestamp_epoch: u64,
    methane_pct_x10: u16,
    co2_pct_x10: u16,
    h2s_ppm: u32,
    total_flow_m3_h_x10: u32,
    gas_holder_level_pct_x10: u16,
    flare_active: bool,
    energy_kwh_generated: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutrientRemovalRecord {
    record_id: u64,
    process: NutrientRemovalProcess,
    influent_tn_mg_l_x100: u32,
    effluent_tn_mg_l_x100: u32,
    influent_tp_mg_l_x100: u32,
    effluent_tp_mg_l_x100: u32,
    chemical_phosphorus_removal: bool,
    ferric_chloride_dose_mg_l_x100: u32,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CorrosionControlIndex {
    sample_id: u64,
    langelier_index_x1000: i32,
    ryznar_index_x1000: u32,
    calcium_hardness_mg_l: u32,
    alkalinity_mg_l: u32,
    tds_mg_l: u32,
    ph_x100: u16,
    temperature_c_x10: u16,
    treatment_location: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CustomerComplaint {
    complaint_id: u64,
    category: ComplaintCategory,
    customer_account: String,
    address_zone: u16,
    description: String,
    reported_epoch: u64,
    resolved_epoch: u64,
    resolution_notes: String,
    resolved: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_influent_quality_sample_roundtrip() {
    let sample = InfluentQualitySample {
        sample_id: 10001,
        timestamp_epoch: 1_710_000_000,
        bod_mg_l: 220,
        cod_mg_l: 480,
        tss_mg_l: 250,
        ph_x100: 710,
        turbidity_ntu_x10: 1500,
        ammonia_mg_l_x100: 2800,
        flow_rate_m3_h: 4500,
        temperature_c_x10: 185,
    };
    let bytes = encode_to_vec(&sample).expect("encode InfluentQualitySample failed");
    let (decoded, consumed): (InfluentQualitySample, usize) =
        decode_from_slice(&bytes).expect("decode InfluentQualitySample failed");
    assert_eq!(sample, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_influent_quality_versioned_v1_0_0() {
    let sample = InfluentQualitySample {
        sample_id: 10002,
        timestamp_epoch: 1_710_100_000,
        bod_mg_l: 310,
        cod_mg_l: 650,
        tss_mg_l: 380,
        ph_x100: 690,
        turbidity_ntu_x10: 2200,
        ammonia_mg_l_x100: 3500,
        flow_rate_m3_h: 6000,
        temperature_c_x10: 210,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sample, version)
        .expect("encode versioned InfluentQualitySample v1.0.0 failed");
    let (decoded, ver, consumed): (InfluentQualitySample, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned InfluentQualitySample v1.0.0 failed");
    assert_eq!(sample, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_effluent_quality_compliant_roundtrip() {
    let sample = EffluentQualitySample {
        sample_id: 20001,
        timestamp_epoch: 1_710_200_000,
        bod_mg_l: 8,
        cod_mg_l: 25,
        tss_mg_l: 10,
        ph_x100: 720,
        turbidity_ntu_x10: 15,
        total_nitrogen_mg_l_x100: 800,
        total_phosphorus_mg_l_x100: 50,
        fecal_coliform_cfu_100ml: 5,
        compliant: true,
    };
    let bytes = encode_to_vec(&sample).expect("encode EffluentQualitySample failed");
    let (decoded, consumed): (EffluentQualitySample, usize) =
        decode_from_slice(&bytes).expect("decode EffluentQualitySample failed");
    assert_eq!(sample, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_effluent_quality_versioned_v2_1_0() {
    let sample = EffluentQualitySample {
        sample_id: 20002,
        timestamp_epoch: 1_710_300_000,
        bod_mg_l: 15,
        cod_mg_l: 45,
        tss_mg_l: 22,
        ph_x100: 680,
        turbidity_ntu_x10: 35,
        total_nitrogen_mg_l_x100: 1500,
        total_phosphorus_mg_l_x100: 120,
        fecal_coliform_cfu_100ml: 180,
        compliant: false,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&sample, version)
        .expect("encode versioned EffluentQualitySample v2.1.0 failed");
    let (decoded, ver, _consumed): (EffluentQualitySample, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned EffluentQualitySample v2.1.0 failed");
    assert_eq!(sample, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_chemical_dosing_record_roundtrip() {
    let record = ChemicalDosingRecord {
        record_id: 30001,
        chemical_name: "Ferric Chloride (FeCl3)".to_string(),
        dosage_mg_l_x100: 4500,
        flow_rate_gph: 120,
        injection_point: "Primary Clarifier Inlet".to_string(),
        operator_id: "OP-2847".to_string(),
        timestamp_epoch: 1_710_400_000,
        tank_level_pct_x10: 682,
        batch_lot_number: "FC-2024-0319-A".to_string(),
    };
    let bytes = encode_to_vec(&record).expect("encode ChemicalDosingRecord failed");
    let (decoded, consumed): (ChemicalDosingRecord, usize) =
        decode_from_slice(&bytes).expect("decode ChemicalDosingRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_chemical_dosing_versioned_v1_2_3() {
    let record = ChemicalDosingRecord {
        record_id: 30002,
        chemical_name: "Sodium Hypochlorite (NaOCl)".to_string(),
        dosage_mg_l_x100: 250,
        flow_rate_gph: 85,
        injection_point: "Post-Filtration Contact Tank".to_string(),
        operator_id: "OP-1193".to_string(),
        timestamp_epoch: 1_710_500_000,
        tank_level_pct_x10: 445,
        batch_lot_number: "SH-2024-0320-C".to_string(),
    };
    let version = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned ChemicalDosingRecord v1.2.3 failed");
    let (decoded, ver, consumed): (ChemicalDosingRecord, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned ChemicalDosingRecord v1.2.3 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_filtration_system_online_roundtrip() {
    let status = FiltrationSystemStatus {
        filter_id: 4,
        media_type: FilterMediaType::DualMedia,
        head_loss_m_x100: 185,
        flow_rate_m3_h_x10: 3200,
        turbidity_out_ntu_x100: 8,
        run_hours_since_backwash: 36,
        backwash_count: 142,
        last_backwash_epoch: 1_710_350_000,
        online: true,
    };
    let bytes = encode_to_vec(&status).expect("encode FiltrationSystemStatus failed");
    let (decoded, consumed): (FiltrationSystemStatus, usize) =
        decode_from_slice(&bytes).expect("decode FiltrationSystemStatus failed");
    assert_eq!(status, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_filtration_system_versioned_v2_0_0() {
    let status = FiltrationSystemStatus {
        filter_id: 9,
        media_type: FilterMediaType::ActivatedCarbon,
        head_loss_m_x100: 310,
        flow_rate_m3_h_x10: 2100,
        turbidity_out_ntu_x100: 3,
        run_hours_since_backwash: 72,
        backwash_count: 288,
        last_backwash_epoch: 1_710_550_000,
        online: false,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&status, version)
        .expect("encode versioned FiltrationSystemStatus v2.0.0 failed");
    let (decoded, ver, consumed): (FiltrationSystemStatus, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned FiltrationSystemStatus v2.0.0 failed");
    assert_eq!(status, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
    assert!(!decoded.online);
}

#[test]
fn test_uv_disinfection_versioned_v3_0_0() {
    let reading = UvDisinfectionReading {
        unit_id: 7,
        timestamp_epoch: 1_710_600_000,
        dose_mj_cm2_x100: 4000,
        transmittance_pct_x10: 920,
        lamp_hours: 8760,
        lamp_intensity_pct_x10: 875,
        flow_rate_m3_h: 2200,
        alarm_active: false,
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&reading, version)
        .expect("encode versioned UvDisinfectionReading v3.0.0 failed");
    let (decoded, ver, consumed): (UvDisinfectionReading, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned UvDisinfectionReading v3.0.0 failed");
    assert_eq!(reading, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_mbr_performance_high_pressure_roundtrip() {
    let perf = MbrPerformance {
        train_id: 2,
        timestamp_epoch: 1_710_700_000,
        trans_membrane_pressure_kpa_x10: 450,
        permeate_flux_lmh_x100: 2500,
        mlss_mg_l: 10_000,
        srt_days: 20,
        dissolved_oxygen_mg_l_x100: 200,
        permeate_turbidity_ntu_x1000: 50,
        chemical_clean_count: 8,
    };
    let bytes = encode_to_vec(&perf).expect("encode MbrPerformance failed");
    let (decoded, consumed): (MbrPerformance, usize) =
        decode_from_slice(&bytes).expect("decode MbrPerformance failed");
    assert_eq!(perf, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_mbr_performance_versioned_v1_5_0() {
    let perf = MbrPerformance {
        train_id: 3,
        timestamp_epoch: 1_710_800_000,
        trans_membrane_pressure_kpa_x10: 280,
        permeate_flux_lmh_x100: 3100,
        mlss_mg_l: 8_500,
        srt_days: 15,
        dissolved_oxygen_mg_l_x100: 180,
        permeate_turbidity_ntu_x1000: 30,
        chemical_clean_count: 3,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&perf, version)
        .expect("encode versioned MbrPerformance v1.5.0 failed");
    let (decoded, ver, consumed): (MbrPerformance, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned MbrPerformance v1.5.0 failed");
    assert_eq!(perf, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_sludge_processing_params_roundtrip() {
    let params = SludgeProcessingParams {
        batch_id: 50001,
        total_solids_pct_x100: 450,
        volatile_solids_pct_x100: 320,
        dewatered_cake_pct_x100: 2200,
        polymer_dose_kg_dry_ton_x10: 85,
        digester_temp_c_x10: 370,
        retention_time_days: 21,
        biogas_m3_h_x10: 1500,
        disposal_method: "Land Application Class B".to_string(),
    };
    let bytes = encode_to_vec(&params).expect("encode SludgeProcessingParams failed");
    let (decoded, consumed): (SludgeProcessingParams, usize) =
        decode_from_slice(&bytes).expect("decode SludgeProcessingParams failed");
    assert_eq!(params, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_scada_alarm_critical_versioned_v2_3_1() {
    let alarm = ScadaAlarmEvent {
        alarm_id: 60001,
        timestamp_epoch: 1_710_900_000,
        severity: AlarmSeverity::Critical,
        source_tag: "FIT-301A".to_string(),
        description: "High influent flow exceeds plant capacity threshold".to_string(),
        value_at_trigger: 7200,
        setpoint: 6500,
        acknowledged: false,
        resolved_epoch: 0,
    };
    let version = Version::new(2, 3, 1);
    let bytes = encode_versioned_value(&alarm, version)
        .expect("encode versioned ScadaAlarmEvent v2.3.1 failed");
    let (decoded, ver, consumed): (ScadaAlarmEvent, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned ScadaAlarmEvent v2.3.1 failed");
    assert_eq!(alarm, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_scada_alarm_emergency_roundtrip() {
    let alarm = ScadaAlarmEvent {
        alarm_id: 60002,
        timestamp_epoch: 1_711_000_000,
        severity: AlarmSeverity::Emergency,
        source_tag: "CL2-RES-001".to_string(),
        description: "Chlorine gas leak detected in chemical storage area".to_string(),
        value_at_trigger: 1,
        setpoint: 0,
        acknowledged: true,
        resolved_epoch: 1_711_003_600,
    };
    let bytes = encode_to_vec(&alarm).expect("encode ScadaAlarmEvent Emergency failed");
    let (decoded, consumed): (ScadaAlarmEvent, usize) =
        decode_from_slice(&bytes).expect("decode ScadaAlarmEvent Emergency failed");
    assert_eq!(alarm, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_regulatory_compliance_sample_versioned_v1_0_0() {
    let sample = RegulatoryComplianceSample {
        sample_id: 70001,
        permit_id: "NPDES-CA0037681".to_string(),
        parameter_name: "BOD5".to_string(),
        measured_value_x1000: 12_500,
        permit_limit_x1000: 30_000,
        unit: "mg/L".to_string(),
        sampling_location: "Outfall 001".to_string(),
        lab_id: "ENV-LAB-4421".to_string(),
        compliant: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sample, version)
        .expect("encode versioned RegulatoryComplianceSample v1.0.0 failed");
    let (decoded, ver, consumed): (RegulatoryComplianceSample, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned RegulatoryComplianceSample v1.0.0 failed");
    assert_eq!(sample, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_distribution_network_reading_roundtrip() {
    let reading = DistributionNetworkReading {
        node_id: 1042,
        timestamp_epoch: 1_711_100_000,
        pressure_psi_x100: 6520,
        flow_gpm_x10: 8400,
        chlorine_residual_mg_l_x100: 85,
        temperature_c_x10: 155,
        zone_id: 7,
        valve_status_open: true,
    };
    let bytes = encode_to_vec(&reading).expect("encode DistributionNetworkReading failed");
    let (decoded, consumed): (DistributionNetworkReading, usize) =
        decode_from_slice(&bytes).expect("decode DistributionNetworkReading failed");
    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_stormwater_overflow_versioned_v4_0_0() {
    let event = StormwaterOverflowEvent {
        event_id: 80001,
        outfall_id: "CSO-017".to_string(),
        start_epoch: 1_711_200_000,
        end_epoch: 1_711_214_400,
        volume_gallons: 2_850_000,
        peak_flow_gpm: 12_500,
        rainfall_inches_x100: 275,
        receiving_water: "Cedar Creek".to_string(),
        reported_to_agency: true,
    };
    let version = Version::new(4, 0, 0);
    let bytes = encode_versioned_value(&event, version)
        .expect("encode versioned StormwaterOverflowEvent v4.0.0 failed");
    let (decoded, ver, _consumed): (StormwaterOverflowEvent, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned StormwaterOverflowEvent v4.0.0 failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_biogas_generation_record_roundtrip() {
    let record = BiogasGenerationRecord {
        digester_id: 2,
        timestamp_epoch: 1_711_300_000,
        methane_pct_x10: 630,
        co2_pct_x10: 340,
        h2s_ppm: 450,
        total_flow_m3_h_x10: 2800,
        gas_holder_level_pct_x10: 720,
        flare_active: false,
        energy_kwh_generated: 1850,
    };
    let bytes = encode_to_vec(&record).expect("encode BiogasGenerationRecord failed");
    let (decoded, consumed): (BiogasGenerationRecord, usize) =
        decode_from_slice(&bytes).expect("decode BiogasGenerationRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_nutrient_removal_versioned_v2_0_0() {
    let record = NutrientRemovalRecord {
        record_id: 90001,
        process: NutrientRemovalProcess::ModifiedBardenpho,
        influent_tn_mg_l_x100: 4200,
        effluent_tn_mg_l_x100: 350,
        influent_tp_mg_l_x100: 800,
        effluent_tp_mg_l_x100: 50,
        chemical_phosphorus_removal: true,
        ferric_chloride_dose_mg_l_x100: 3500,
        timestamp_epoch: 1_711_400_000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned NutrientRemovalRecord v2.0.0 failed");
    let (decoded, ver, consumed): (NutrientRemovalRecord, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned NutrientRemovalRecord v2.0.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_corrosion_control_index_negative_lsi_roundtrip() {
    let index = CorrosionControlIndex {
        sample_id: 100001,
        langelier_index_x1000: -450,
        ryznar_index_x1000: 8_900,
        calcium_hardness_mg_l: 85,
        alkalinity_mg_l: 60,
        tds_mg_l: 320,
        ph_x100: 720,
        temperature_c_x10: 180,
        treatment_location: "Distribution Entry Point DEP-03".to_string(),
    };
    let bytes = encode_to_vec(&index).expect("encode CorrosionControlIndex failed");
    let (decoded, consumed): (CorrosionControlIndex, usize) =
        decode_from_slice(&bytes).expect("decode CorrosionControlIndex failed");
    assert_eq!(index, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(
        decoded.langelier_index_x1000 < 0,
        "LSI should be negative for corrosive water"
    );
}

#[test]
fn test_corrosion_control_versioned_v1_1_0() {
    let index = CorrosionControlIndex {
        sample_id: 100002,
        langelier_index_x1000: 250,
        ryznar_index_x1000: 5_500,
        calcium_hardness_mg_l: 200,
        alkalinity_mg_l: 150,
        tds_mg_l: 480,
        ph_x100: 780,
        temperature_c_x10: 220,
        treatment_location: "WTP Clear Well Outlet".to_string(),
    };
    let version = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&index, version)
        .expect("encode versioned CorrosionControlIndex v1.1.0 failed");
    let (decoded, ver, consumed): (CorrosionControlIndex, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned CorrosionControlIndex v1.1.0 failed");
    assert_eq!(index, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
    assert!(
        decoded.langelier_index_x1000 > 0,
        "LSI should be positive for scale-forming water"
    );
}

#[test]
fn test_customer_complaint_resolved_versioned_v3_2_1() {
    let complaint = CustomerComplaint {
        complaint_id: 110001,
        category: ComplaintCategory::Discoloration,
        customer_account: "ACCT-2024-88712".to_string(),
        address_zone: 12,
        description: "Brown water coming from kitchen tap after hydrant flushing nearby"
            .to_string(),
        reported_epoch: 1_711_500_000,
        resolved_epoch: 1_711_586_400,
        resolution_notes: "Main flushed, dead-end eliminated, customer confirmed clear water"
            .to_string(),
        resolved: true,
    };
    let version = Version::new(3, 2, 1);
    let bytes = encode_versioned_value(&complaint, version)
        .expect("encode versioned CustomerComplaint v3.2.1 failed");
    let (decoded, ver, consumed): (CustomerComplaint, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned CustomerComplaint v3.2.1 failed");
    assert_eq!(complaint, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 1);
    assert!(consumed > 0);
    assert!(decoded.resolved);
}
