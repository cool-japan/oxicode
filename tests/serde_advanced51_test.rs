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

// --- Domain types: Mining and Geological Exploration ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DrillCoreSample {
    hole_id: String,
    depth_from_m: f64,
    depth_to_m: f64,
    lithology: String,
    recovery_pct: f32,
    rqd_pct: f32,
    core_diameter_mm: f64,
    orientation_azimuth: f64,
    orientation_dip: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AssayResult {
    sample_id: String,
    hole_id: String,
    depth_from_m: f64,
    depth_to_m: f64,
    au_g_per_t: f64,
    ag_g_per_t: f64,
    cu_pct: f64,
    fe_pct: f64,
    s_pct: f64,
    lab_code: String,
    method: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum OreGradeClassification {
    HighGrade {
        cutoff_g_per_t: f64,
        tonnage: f64,
        grade_au: f64,
    },
    MediumGrade {
        cutoff_g_per_t: f64,
        tonnage: f64,
        grade_au: f64,
    },
    LowGrade {
        cutoff_g_per_t: f64,
        tonnage: f64,
        grade_au: f64,
    },
    Waste {
        tonnage: f64,
        destination: String,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BlastPatternDesign {
    pattern_id: String,
    burden_m: f64,
    spacing_m: f64,
    hole_depth_m: f64,
    hole_diameter_mm: f64,
    stemming_length_m: f64,
    subdrill_m: f64,
    powder_factor_kg_per_m3: f64,
    explosive_type: String,
    num_rows: u32,
    holes_per_row: u32,
    initiation_sequence: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MineVentilationParams {
    zone_id: String,
    airflow_m3_per_s: f64,
    velocity_m_per_s: f64,
    pressure_drop_pa: f64,
    temperature_c: f64,
    humidity_pct: f32,
    co_ppm: f64,
    nox_ppm: f64,
    dust_mg_per_m3: f64,
    fan_speed_rpm: u32,
    fan_power_kw: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GeophysicalSurveyData {
    survey_id: String,
    line_id: String,
    station_m: f64,
    easting: f64,
    northing: f64,
    elevation_m: f64,
    seismic_velocity_m_per_s: f64,
    magnetic_nt: f64,
    gravity_mgal: f64,
    resistivity_ohm_m: f64,
    ip_chargeability_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ResourceCategory {
    Measured,
    Indicated,
    Inferred,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MineralResourceEstimate {
    block_id: String,
    category: ResourceCategory,
    tonnage_mt: f64,
    grade_au_g_per_t: f64,
    grade_cu_pct: f64,
    contained_au_oz: f64,
    contained_cu_t: f64,
    density_t_per_m3: f64,
    confidence_level: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TailingsDamMonitoring {
    dam_id: String,
    timestamp_epoch: u64,
    water_level_m: f64,
    freeboard_m: f64,
    phreatic_surface_m: f64,
    pore_pressure_kpa: f64,
    seepage_l_per_min: f64,
    settlement_mm: f64,
    ph_value: f32,
    turbidity_ntu: f64,
    inclinometer_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SlopeStabilityMeasurement {
    sector_id: String,
    bench_height_m: f64,
    bench_angle_deg: f64,
    inter_ramp_angle_deg: f64,
    overall_slope_angle_deg: f64,
    factor_of_safety: f64,
    displacement_mm: f64,
    displacement_rate_mm_per_day: f64,
    groundwater_level_m: f64,
    joint_set_orientations: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UndergroundTunnelSurvey {
    tunnel_id: String,
    chainage_m: f64,
    width_m: f64,
    height_m: f64,
    cross_section_area_m2: f64,
    azimuth_deg: f64,
    gradient_pct: f64,
    rock_class: String,
    support_type: String,
    convergence_mm: f64,
    overbreak_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CrushingGrindingCircuit {
    circuit_id: String,
    feed_rate_tph: f64,
    feed_size_p80_mm: f64,
    product_size_p80_um: f64,
    crusher_power_kw: f64,
    sag_mill_power_kw: f64,
    ball_mill_power_kw: f64,
    bond_work_index: f64,
    circulating_load_pct: f64,
    cyclone_overflow_pct_passing: f64,
    specific_energy_kwh_per_t: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FlotationCellReading {
    cell_id: String,
    bank_name: String,
    feed_grade_pct: f64,
    concentrate_grade_pct: f64,
    tailing_grade_pct: f64,
    recovery_pct: f64,
    air_flow_l_per_min: f64,
    froth_depth_mm: f64,
    reagent_dosage_g_per_t: f64,
    ph_value: f32,
    pulp_density_pct: f64,
    impeller_speed_rpm: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeapLeachPadMonitoring {
    pad_id: String,
    lift_number: u32,
    ore_tonnage_on_pad: f64,
    head_grade_au_g_per_t: f64,
    solution_application_rate_l_per_m2_hr: f64,
    pregnant_solution_grade_mg_per_l: f64,
    barren_solution_grade_mg_per_l: f64,
    extraction_pct: f64,
    days_under_leach: u32,
    liner_integrity_ok: bool,
    drain_flow_l_per_min: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EnvironmentalComplianceMetric {
    site_id: String,
    parameter: String,
    measured_value: f64,
    regulatory_limit: f64,
    unit: String,
    compliant: bool,
    monitoring_point: String,
    sample_date_epoch: u64,
    lab_accreditation: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BlockModelCell {
    block_ix: u32,
    block_iy: u32,
    block_iz: u32,
    centroid_x: f64,
    centroid_y: f64,
    centroid_z: f64,
    size_x_m: f64,
    size_y_m: f64,
    size_z_m: f64,
    tonnage: f64,
    grade_au: f64,
    grade_cu: f64,
    rock_type_code: u16,
    mineable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EquipmentFleetDispatch {
    equipment_id: String,
    equipment_type: String,
    operator_id: String,
    current_location: String,
    assigned_task: String,
    payload_tonnes: f64,
    fuel_level_pct: f32,
    engine_hours: f64,
    speed_km_per_h: f64,
    heading_deg: f64,
    gps_easting: f64,
    gps_northing: f64,
    status_code: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DrillHolePlan {
    hole_id: String,
    collar_easting: f64,
    collar_northing: f64,
    collar_elevation: f64,
    planned_azimuth: f64,
    planned_dip: f64,
    planned_depth_m: f64,
    purpose: String,
    target_zone: String,
    rig_type: String,
    estimated_days: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MinePlanPeriod {
    period_id: String,
    year: u16,
    quarter: u8,
    planned_ore_tonnes: f64,
    planned_waste_tonnes: f64,
    strip_ratio: f64,
    planned_grade_au: f64,
    mining_cost_per_tonne: f64,
    processing_cost_per_tonne: f64,
    target_pit_stage: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SeismicEvent {
    event_id: u64,
    timestamp_epoch: u64,
    magnitude: f64,
    easting: f64,
    northing: f64,
    depth_m: f64,
    energy_joules: f64,
    source_mechanism: String,
    num_triggers: u32,
    location_error_m: f64,
}

// --- Tests ---

#[test]
fn test_drill_core_sample_roundtrip() {
    let sample = DrillCoreSample {
        hole_id: "DDH-2026-0047".to_string(),
        depth_from_m: 142.50,
        depth_to_m: 143.80,
        lithology: "Quartz-sericite schist".to_string(),
        recovery_pct: 98.5,
        rqd_pct: 82.0,
        core_diameter_mm: 63.5,
        orientation_azimuth: 225.0,
        orientation_dip: -65.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&sample, cfg).expect("encode drill core sample");
    let (decoded, _): (DrillCoreSample, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode drill core sample");
    assert_eq!(sample, decoded);
}

#[test]
fn test_assay_results_roundtrip() {
    let results = vec![
        AssayResult {
            sample_id: "AS-47-001".to_string(),
            hole_id: "DDH-2026-0047".to_string(),
            depth_from_m: 142.50,
            depth_to_m: 143.80,
            au_g_per_t: 3.72,
            ag_g_per_t: 12.5,
            cu_pct: 0.45,
            fe_pct: 8.2,
            s_pct: 2.1,
            lab_code: "ALS-Perth".to_string(),
            method: "FA-AAS".to_string(),
        },
        AssayResult {
            sample_id: "AS-47-002".to_string(),
            hole_id: "DDH-2026-0047".to_string(),
            depth_from_m: 143.80,
            depth_to_m: 145.30,
            au_g_per_t: 1.15,
            ag_g_per_t: 5.8,
            cu_pct: 0.22,
            fe_pct: 6.7,
            s_pct: 1.4,
            lab_code: "ALS-Perth".to_string(),
            method: "FA-AAS".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&results, cfg).expect("encode assay results");
    let (decoded, _): (Vec<AssayResult>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode assay results");
    assert_eq!(results, decoded);
}

#[test]
fn test_ore_grade_classification_roundtrip() {
    let grades = vec![
        OreGradeClassification::HighGrade {
            cutoff_g_per_t: 3.0,
            tonnage: 1_250_000.0,
            grade_au: 5.8,
        },
        OreGradeClassification::MediumGrade {
            cutoff_g_per_t: 1.5,
            tonnage: 4_800_000.0,
            grade_au: 2.3,
        },
        OreGradeClassification::LowGrade {
            cutoff_g_per_t: 0.5,
            tonnage: 12_000_000.0,
            grade_au: 0.85,
        },
        OreGradeClassification::Waste {
            tonnage: 45_000_000.0,
            destination: "West waste dump".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&grades, cfg).expect("encode ore grade classifications");
    let (decoded, _): (Vec<OreGradeClassification>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ore grade classifications");
    assert_eq!(grades, decoded);
}

#[test]
fn test_blast_pattern_design_roundtrip() {
    let pattern = BlastPatternDesign {
        pattern_id: "BP-2026-0312".to_string(),
        burden_m: 3.8,
        spacing_m: 4.5,
        hole_depth_m: 12.0,
        hole_diameter_mm: 127.0,
        stemming_length_m: 3.0,
        subdrill_m: 1.2,
        powder_factor_kg_per_m3: 0.85,
        explosive_type: "ANFO Heavy".to_string(),
        num_rows: 5,
        holes_per_row: 12,
        initiation_sequence: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&pattern, cfg).expect("encode blast pattern");
    let (decoded, _): (BlastPatternDesign, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode blast pattern");
    assert_eq!(pattern, decoded);
}

#[test]
fn test_mine_ventilation_params_roundtrip() {
    let vent = MineVentilationParams {
        zone_id: "L3-NW-STOPE-12".to_string(),
        airflow_m3_per_s: 45.2,
        velocity_m_per_s: 1.8,
        pressure_drop_pa: 320.0,
        temperature_c: 28.5,
        humidity_pct: 85.0,
        co_ppm: 12.5,
        nox_ppm: 3.2,
        dust_mg_per_m3: 1.8,
        fan_speed_rpm: 1450,
        fan_power_kw: 185.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&vent, cfg).expect("encode ventilation params");
    let (decoded, _): (MineVentilationParams, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ventilation params");
    assert_eq!(vent, decoded);
}

#[test]
fn test_geophysical_survey_data_roundtrip() {
    let readings: Vec<GeophysicalSurveyData> = (0..5)
        .map(|i| GeophysicalSurveyData {
            survey_id: "GEO-MAG-2026-003".to_string(),
            line_id: format!("L{}", 1000 + i * 100),
            station_m: 50.0 * i as f64,
            easting: 456_000.0 + i as f64 * 25.0,
            northing: 7_123_000.0,
            elevation_m: 450.0 + i as f64 * 0.5,
            seismic_velocity_m_per_s: 4800.0 + i as f64 * 120.0,
            magnetic_nt: 52_300.0 + i as f64 * 15.0,
            gravity_mgal: 978_100.0 + i as f64 * 0.3,
            resistivity_ohm_m: 250.0 + i as f64 * 50.0,
            ip_chargeability_ms: 18.0 + i as f64 * 2.0,
        })
        .collect();
    let cfg = config::standard();
    let encoded = encode_to_vec(&readings, cfg).expect("encode geophysical survey");
    let (decoded, _): (Vec<GeophysicalSurveyData>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode geophysical survey");
    assert_eq!(readings, decoded);
}

#[test]
fn test_mineral_resource_estimate_roundtrip() {
    let estimates = vec![
        MineralResourceEstimate {
            block_id: "BLK-M-001".to_string(),
            category: ResourceCategory::Measured,
            tonnage_mt: 2.5,
            grade_au_g_per_t: 4.2,
            grade_cu_pct: 0.35,
            contained_au_oz: 337_500.0,
            contained_cu_t: 8_750.0,
            density_t_per_m3: 2.75,
            confidence_level: 0.95,
        },
        MineralResourceEstimate {
            block_id: "BLK-I-002".to_string(),
            category: ResourceCategory::Indicated,
            tonnage_mt: 8.3,
            grade_au_g_per_t: 2.8,
            grade_cu_pct: 0.28,
            contained_au_oz: 747_000.0,
            contained_cu_t: 23_240.0,
            density_t_per_m3: 2.70,
            confidence_level: 0.80,
        },
        MineralResourceEstimate {
            block_id: "BLK-IF-003".to_string(),
            category: ResourceCategory::Inferred,
            tonnage_mt: 15.0,
            grade_au_g_per_t: 1.9,
            grade_cu_pct: 0.18,
            contained_au_oz: 916_200.0,
            contained_cu_t: 27_000.0,
            density_t_per_m3: 2.65,
            confidence_level: 0.55,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&estimates, cfg).expect("encode resource estimates");
    let (decoded, _): (Vec<MineralResourceEstimate>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode resource estimates");
    assert_eq!(estimates, decoded);
}

#[test]
fn test_tailings_dam_monitoring_roundtrip() {
    let reading = TailingsDamMonitoring {
        dam_id: "TSF-01".to_string(),
        timestamp_epoch: 1_773_676_800,
        water_level_m: 342.8,
        freeboard_m: 2.5,
        phreatic_surface_m: 338.2,
        pore_pressure_kpa: 185.0,
        seepage_l_per_min: 0.35,
        settlement_mm: 12.5,
        ph_value: 7.2,
        turbidity_ntu: 45.0,
        inclinometer_deg: 0.02,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&reading, cfg).expect("encode tailings dam monitoring");
    let (decoded, _): (TailingsDamMonitoring, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode tailings dam monitoring");
    assert_eq!(reading, decoded);
}

#[test]
fn test_slope_stability_measurement_roundtrip() {
    let measurement = SlopeStabilityMeasurement {
        sector_id: "PIT-NW-S3".to_string(),
        bench_height_m: 15.0,
        bench_angle_deg: 70.0,
        inter_ramp_angle_deg: 50.0,
        overall_slope_angle_deg: 42.0,
        factor_of_safety: 1.35,
        displacement_mm: 4.2,
        displacement_rate_mm_per_day: 0.08,
        groundwater_level_m: 285.0,
        joint_set_orientations: vec![45.0, 135.0, 210.0, 310.0],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&measurement, cfg).expect("encode slope stability");
    let (decoded, _): (SlopeStabilityMeasurement, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode slope stability");
    assert_eq!(measurement, decoded);
}

#[test]
fn test_underground_tunnel_survey_roundtrip() {
    let survey = UndergroundTunnelSurvey {
        tunnel_id: "DEV-L4-XC-07".to_string(),
        chainage_m: 1_245.0,
        width_m: 5.0,
        height_m: 5.5,
        cross_section_area_m2: 24.5,
        azimuth_deg: 172.5,
        gradient_pct: -1.2,
        rock_class: "Class III - Fair".to_string(),
        support_type: "Shotcrete 75mm + Split sets".to_string(),
        convergence_mm: 8.3,
        overbreak_pct: 12.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&survey, cfg).expect("encode tunnel survey");
    let (decoded, _): (UndergroundTunnelSurvey, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode tunnel survey");
    assert_eq!(survey, decoded);
}

#[test]
fn test_crushing_grinding_circuit_roundtrip() {
    let circuit = CrushingGrindingCircuit {
        circuit_id: "CG-MAIN-01".to_string(),
        feed_rate_tph: 2_800.0,
        feed_size_p80_mm: 150.0,
        product_size_p80_um: 106.0,
        crusher_power_kw: 1_200.0,
        sag_mill_power_kw: 8_500.0,
        ball_mill_power_kw: 6_200.0,
        bond_work_index: 14.5,
        circulating_load_pct: 350.0,
        cyclone_overflow_pct_passing: 65.0,
        specific_energy_kwh_per_t: 12.8,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&circuit, cfg).expect("encode crushing grinding circuit");
    let (decoded, _): (CrushingGrindingCircuit, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode crushing grinding circuit");
    assert_eq!(circuit, decoded);
}

#[test]
fn test_flotation_cell_reading_roundtrip() {
    let reading = FlotationCellReading {
        cell_id: "FC-RGH-04".to_string(),
        bank_name: "Rougher Bank A".to_string(),
        feed_grade_pct: 0.42,
        concentrate_grade_pct: 18.5,
        tailing_grade_pct: 0.08,
        recovery_pct: 92.3,
        air_flow_l_per_min: 1_200.0,
        froth_depth_mm: 180.0,
        reagent_dosage_g_per_t: 35.0,
        ph_value: 10.5,
        pulp_density_pct: 32.0,
        impeller_speed_rpm: 280,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&reading, cfg).expect("encode flotation cell reading");
    let (decoded, _): (FlotationCellReading, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode flotation cell reading");
    assert_eq!(reading, decoded);
}

#[test]
fn test_heap_leach_pad_monitoring_roundtrip() {
    let pad = HeapLeachPadMonitoring {
        pad_id: "HLP-03".to_string(),
        lift_number: 7,
        ore_tonnage_on_pad: 3_500_000.0,
        head_grade_au_g_per_t: 0.65,
        solution_application_rate_l_per_m2_hr: 10.0,
        pregnant_solution_grade_mg_per_l: 1.2,
        barren_solution_grade_mg_per_l: 0.005,
        extraction_pct: 72.5,
        days_under_leach: 120,
        liner_integrity_ok: true,
        drain_flow_l_per_min: 450.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&pad, cfg).expect("encode heap leach pad");
    let (decoded, _): (HeapLeachPadMonitoring, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode heap leach pad");
    assert_eq!(pad, decoded);
}

#[test]
fn test_environmental_compliance_metrics_roundtrip() {
    let metrics = vec![
        EnvironmentalComplianceMetric {
            site_id: "MINE-ALPHA".to_string(),
            parameter: "Total Suspended Solids".to_string(),
            measured_value: 12.5,
            regulatory_limit: 25.0,
            unit: "mg/L".to_string(),
            compliant: true,
            monitoring_point: "SW-Discharge-01".to_string(),
            sample_date_epoch: 1_773_676_800,
            lab_accreditation: "NATA-14832".to_string(),
        },
        EnvironmentalComplianceMetric {
            site_id: "MINE-ALPHA".to_string(),
            parameter: "Arsenic".to_string(),
            measured_value: 0.008,
            regulatory_limit: 0.01,
            unit: "mg/L".to_string(),
            compliant: true,
            monitoring_point: "GW-MW-05".to_string(),
            sample_date_epoch: 1_773_676_800,
            lab_accreditation: "NATA-14832".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&metrics, cfg).expect("encode environmental metrics");
    let (decoded, _): (Vec<EnvironmentalComplianceMetric>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode environmental metrics");
    assert_eq!(metrics, decoded);
}

#[test]
fn test_block_model_cells_roundtrip() {
    let blocks: Vec<BlockModelCell> = (0..8)
        .map(|i| BlockModelCell {
            block_ix: i,
            block_iy: i + 10,
            block_iz: 5,
            centroid_x: 456_000.0 + i as f64 * 10.0,
            centroid_y: 7_123_000.0 + i as f64 * 10.0,
            centroid_z: 350.0 - i as f64 * 10.0,
            size_x_m: 10.0,
            size_y_m: 10.0,
            size_z_m: 10.0,
            tonnage: 2_750.0,
            grade_au: 2.1 + i as f64 * 0.3,
            grade_cu: 0.25 + i as f64 * 0.05,
            rock_type_code: 3,
            mineable: i < 6,
        })
        .collect();
    let cfg = config::standard();
    let encoded = encode_to_vec(&blocks, cfg).expect("encode block model cells");
    let (decoded, _): (Vec<BlockModelCell>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode block model cells");
    assert_eq!(blocks, decoded);
}

#[test]
fn test_equipment_fleet_dispatch_roundtrip() {
    let fleet = vec![
        EquipmentFleetDispatch {
            equipment_id: "HT-101".to_string(),
            equipment_type: "CAT 793F Haul Truck".to_string(),
            operator_id: "OP-2234".to_string(),
            current_location: "Pit Stage 3 Loading".to_string(),
            assigned_task: "Ore haulage to ROM pad".to_string(),
            payload_tonnes: 227.0,
            fuel_level_pct: 68.0,
            engine_hours: 14_520.5,
            speed_km_per_h: 35.0,
            heading_deg: 180.0,
            gps_easting: 456_120.0,
            gps_northing: 7_123_450.0,
            status_code: 1,
        },
        EquipmentFleetDispatch {
            equipment_id: "EX-042".to_string(),
            equipment_type: "Liebherr R9800 Excavator".to_string(),
            operator_id: "OP-1187".to_string(),
            current_location: "Pit Stage 3 Face 2".to_string(),
            assigned_task: "Ore loading".to_string(),
            payload_tonnes: 0.0,
            fuel_level_pct: 52.0,
            engine_hours: 9_830.0,
            speed_km_per_h: 0.0,
            heading_deg: 90.0,
            gps_easting: 456_080.0,
            gps_northing: 7_123_420.0,
            status_code: 2,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&fleet, cfg).expect("encode fleet dispatch");
    let (decoded, _): (Vec<EquipmentFleetDispatch>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode fleet dispatch");
    assert_eq!(fleet, decoded);
}

#[test]
fn test_drill_hole_plan_roundtrip() {
    let plans = vec![
        DrillHolePlan {
            hole_id: "DDH-2026-0048".to_string(),
            collar_easting: 456_250.0,
            collar_northing: 7_123_180.0,
            collar_elevation: 465.0,
            planned_azimuth: 225.0,
            planned_dip: -60.0,
            planned_depth_m: 350.0,
            purpose: "Resource definition".to_string(),
            target_zone: "Mineralized zone B".to_string(),
            rig_type: "Diamond DD77".to_string(),
            estimated_days: 14,
        },
        DrillHolePlan {
            hole_id: "RC-2026-0112".to_string(),
            collar_easting: 456_400.0,
            collar_northing: 7_123_300.0,
            collar_elevation: 470.0,
            planned_azimuth: 180.0,
            planned_dip: -90.0,
            planned_depth_m: 120.0,
            purpose: "Grade control".to_string(),
            target_zone: "Pit Stage 4 bench".to_string(),
            rig_type: "RC Schramm T685".to_string(),
            estimated_days: 3,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&plans, cfg).expect("encode drill hole plans");
    let (decoded, _): (Vec<DrillHolePlan>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode drill hole plans");
    assert_eq!(plans, decoded);
}

#[test]
fn test_mine_plan_period_roundtrip() {
    let periods = vec![
        MinePlanPeriod {
            period_id: "MP-2026-Q1".to_string(),
            year: 2026,
            quarter: 1,
            planned_ore_tonnes: 2_500_000.0,
            planned_waste_tonnes: 7_500_000.0,
            strip_ratio: 3.0,
            planned_grade_au: 2.8,
            mining_cost_per_tonne: 3.50,
            processing_cost_per_tonne: 12.80,
            target_pit_stage: "Stage 3 cutback".to_string(),
        },
        MinePlanPeriod {
            period_id: "MP-2026-Q2".to_string(),
            year: 2026,
            quarter: 2,
            planned_ore_tonnes: 2_800_000.0,
            planned_waste_tonnes: 6_200_000.0,
            strip_ratio: 2.21,
            planned_grade_au: 3.1,
            mining_cost_per_tonne: 3.45,
            processing_cost_per_tonne: 12.60,
            target_pit_stage: "Stage 3 base".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&periods, cfg).expect("encode mine plan periods");
    let (decoded, _): (Vec<MinePlanPeriod>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode mine plan periods");
    assert_eq!(periods, decoded);
}

#[test]
fn test_seismic_event_roundtrip() {
    let events = vec![
        SeismicEvent {
            event_id: 104_892,
            timestamp_epoch: 1_773_676_800,
            magnitude: -0.8,
            easting: 456_050.0,
            northing: 7_123_200.0,
            depth_m: 480.0,
            energy_joules: 125.0,
            source_mechanism: "Shear".to_string(),
            num_triggers: 6,
            location_error_m: 5.2,
        },
        SeismicEvent {
            event_id: 104_893,
            timestamp_epoch: 1_773_677_100,
            magnitude: 1.2,
            easting: 456_080.0,
            northing: 7_123_220.0,
            depth_m: 510.0,
            energy_joules: 15_800.0,
            source_mechanism: "Tensile".to_string(),
            num_triggers: 12,
            location_error_m: 3.1,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&events, cfg).expect("encode seismic events");
    let (decoded, _): (Vec<SeismicEvent>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode seismic events");
    assert_eq!(events, decoded);
}

#[test]
fn test_nested_core_with_assay_roundtrip() {
    let combined: (DrillCoreSample, Vec<AssayResult>) = (
        DrillCoreSample {
            hole_id: "DDH-2026-0050".to_string(),
            depth_from_m: 200.0,
            depth_to_m: 201.5,
            lithology: "Banded iron formation".to_string(),
            recovery_pct: 100.0,
            rqd_pct: 90.0,
            core_diameter_mm: 63.5,
            orientation_azimuth: 180.0,
            orientation_dip: -70.0,
        },
        vec![
            AssayResult {
                sample_id: "AS-50-101".to_string(),
                hole_id: "DDH-2026-0050".to_string(),
                depth_from_m: 200.0,
                depth_to_m: 200.75,
                au_g_per_t: 8.4,
                ag_g_per_t: 22.0,
                cu_pct: 0.78,
                fe_pct: 12.0,
                s_pct: 3.5,
                lab_code: "SGS-Townsville".to_string(),
                method: "FA-GRAV".to_string(),
            },
            AssayResult {
                sample_id: "AS-50-102".to_string(),
                hole_id: "DDH-2026-0050".to_string(),
                depth_from_m: 200.75,
                depth_to_m: 201.5,
                au_g_per_t: 6.1,
                ag_g_per_t: 18.5,
                cu_pct: 0.62,
                fe_pct: 11.2,
                s_pct: 2.9,
                lab_code: "SGS-Townsville".to_string(),
                method: "FA-GRAV".to_string(),
            },
        ],
    );
    let cfg = config::standard();
    let encoded = encode_to_vec(&combined, cfg).expect("encode core with assays");
    let (decoded, _): ((DrillCoreSample, Vec<AssayResult>), _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode core with assays");
    assert_eq!(combined, decoded);
}

#[test]
fn test_multiple_ventilation_zones_roundtrip() {
    let zones: Vec<MineVentilationParams> = (0..4)
        .map(|i| MineVentilationParams {
            zone_id: format!("L{}-ZONE-{}", 3 + i, i + 1),
            airflow_m3_per_s: 30.0 + i as f64 * 10.0,
            velocity_m_per_s: 1.2 + i as f64 * 0.3,
            pressure_drop_pa: 200.0 + i as f64 * 80.0,
            temperature_c: 26.0 + i as f64 * 1.5,
            humidity_pct: 75.0 + i as f32 * 5.0,
            co_ppm: 8.0 + i as f64 * 2.0,
            nox_ppm: 2.0 + i as f64 * 0.5,
            dust_mg_per_m3: 1.0 + i as f64 * 0.4,
            fan_speed_rpm: 1200 + i * 100,
            fan_power_kw: 120.0 + i as f64 * 30.0,
        })
        .collect();
    let cfg = config::standard();
    let encoded = encode_to_vec(&zones, cfg).expect("encode ventilation zones");
    let (decoded, _): (Vec<MineVentilationParams>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ventilation zones");
    assert_eq!(zones, decoded);
}

#[test]
fn test_full_mine_snapshot_roundtrip() {
    let snapshot: (
        Vec<BlockModelCell>,
        Vec<EquipmentFleetDispatch>,
        TailingsDamMonitoring,
        Vec<EnvironmentalComplianceMetric>,
    ) = (
        vec![BlockModelCell {
            block_ix: 0,
            block_iy: 0,
            block_iz: 0,
            centroid_x: 456_005.0,
            centroid_y: 7_123_005.0,
            centroid_z: 395.0,
            size_x_m: 10.0,
            size_y_m: 10.0,
            size_z_m: 10.0,
            tonnage: 2_750.0,
            grade_au: 3.4,
            grade_cu: 0.42,
            rock_type_code: 2,
            mineable: true,
        }],
        vec![EquipmentFleetDispatch {
            equipment_id: "WC-007".to_string(),
            equipment_type: "CAT 777G Water Cart".to_string(),
            operator_id: "OP-3301".to_string(),
            current_location: "Haul Road Section 4".to_string(),
            assigned_task: "Dust suppression".to_string(),
            payload_tonnes: 72.0,
            fuel_level_pct: 45.0,
            engine_hours: 6_200.0,
            speed_km_per_h: 28.0,
            heading_deg: 270.0,
            gps_easting: 456_500.0,
            gps_northing: 7_123_800.0,
            status_code: 1,
        }],
        TailingsDamMonitoring {
            dam_id: "TSF-02".to_string(),
            timestamp_epoch: 1_773_680_400,
            water_level_m: 340.1,
            freeboard_m: 3.0,
            phreatic_surface_m: 336.0,
            pore_pressure_kpa: 170.0,
            seepage_l_per_min: 0.28,
            settlement_mm: 10.0,
            ph_value: 7.5,
            turbidity_ntu: 35.0,
            inclinometer_deg: 0.01,
        },
        vec![EnvironmentalComplianceMetric {
            site_id: "MINE-ALPHA".to_string(),
            parameter: "pH".to_string(),
            measured_value: 7.4,
            regulatory_limit: 8.5,
            unit: "pH units".to_string(),
            compliant: true,
            monitoring_point: "SW-Discharge-02".to_string(),
            sample_date_epoch: 1_773_680_400,
            lab_accreditation: "NATA-14832".to_string(),
        }],
    );
    let cfg = config::standard();
    let encoded = encode_to_vec(&snapshot, cfg).expect("encode full mine snapshot");
    let (decoded, _): (
        (
            Vec<BlockModelCell>,
            Vec<EquipmentFleetDispatch>,
            TailingsDamMonitoring,
            Vec<EnvironmentalComplianceMetric>,
        ),
        _,
    ) = decode_owned_from_slice(&encoded, cfg).expect("decode full mine snapshot");
    assert_eq!(snapshot, decoded);
}
