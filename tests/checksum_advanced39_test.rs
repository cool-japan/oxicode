//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Theme: paper and pulp mill operations.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced39_test

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
// Test 1: Wood chip quality grade
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct WoodChipQualityGrade {
    species: String,
    moisture_content_pct: f64,
    bark_content_pct: f64,
    average_chip_length_mm: f64,
    fines_fraction_pct: f64,
    overthick_fraction_pct: f64,
    grade: String,
}

#[test]
fn test_wood_chip_quality_grade() {
    let sample = WoodChipQualityGrade {
        species: "Eucalyptus grandis".to_string(),
        moisture_content_pct: 48.3,
        bark_content_pct: 1.7,
        average_chip_length_mm: 22.5,
        fines_fraction_pct: 3.2,
        overthick_fraction_pct: 5.1,
        grade: "A-Premium".to_string(),
    };
    let encoded = encode_with_checksum(&sample).expect("encode wood chip quality grade failed");
    let (decoded, consumed): (WoodChipQualityGrade, _) =
        decode_with_checksum(&encoded).expect("decode wood chip quality grade failed");
    assert_eq!(decoded, sample, "wood chip quality grade mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: Digester cooking parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct DigesterCookingParams {
    digester_id: u32,
    cooking_temperature_c: f64,
    cooking_pressure_kpa: f64,
    h_factor_target: u32,
    active_alkali_charge_pct: f64,
    sulfidity_pct: f64,
    liquor_to_wood_ratio: f64,
    cook_duration_minutes: u32,
    blow_temperature_c: f64,
}

#[test]
fn test_digester_cooking_params() {
    let params = DigesterCookingParams {
        digester_id: 3,
        cooking_temperature_c: 170.0,
        cooking_pressure_kpa: 850.0,
        h_factor_target: 1600,
        active_alkali_charge_pct: 18.5,
        sulfidity_pct: 28.0,
        liquor_to_wood_ratio: 3.5,
        cook_duration_minutes: 240,
        blow_temperature_c: 100.0,
    };
    let encoded = encode_with_checksum(&params).expect("encode digester cooking params failed");
    let (decoded, consumed): (DigesterCookingParams, _) =
        decode_with_checksum(&encoded).expect("decode digester cooking params failed");
    assert_eq!(decoded, params, "digester cooking params mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: Bleaching sequence stage
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum BleachingChemical {
    ChlorineDioxide,
    Oxygen,
    Hydrogen,
    Ozone,
    PeraceticAcid,
    CausticExtraction,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BleachingStage {
    stage_index: u8,
    chemical: BleachingChemical,
    temperature_c: f64,
    retention_time_min: u32,
    consistency_pct: f64,
    ph_target: f64,
    chemical_charge_kg_per_ton: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BleachingSequence {
    sequence_name: String,
    stages: Vec<BleachingStage>,
    target_brightness_iso: f64,
}

#[test]
fn test_bleaching_sequence() {
    let seq = BleachingSequence {
        sequence_name: "D0-EOP-D1-EP-D2".to_string(),
        stages: vec![
            BleachingStage {
                stage_index: 0,
                chemical: BleachingChemical::ChlorineDioxide,
                temperature_c: 55.0,
                retention_time_min: 45,
                consistency_pct: 10.0,
                ph_target: 2.5,
                chemical_charge_kg_per_ton: 18.0,
            },
            BleachingStage {
                stage_index: 1,
                chemical: BleachingChemical::CausticExtraction,
                temperature_c: 72.0,
                retention_time_min: 60,
                consistency_pct: 10.0,
                ph_target: 11.0,
                chemical_charge_kg_per_ton: 12.0,
            },
            BleachingStage {
                stage_index: 2,
                chemical: BleachingChemical::ChlorineDioxide,
                temperature_c: 70.0,
                retention_time_min: 180,
                consistency_pct: 10.0,
                ph_target: 3.8,
                chemical_charge_kg_per_ton: 8.5,
            },
        ],
        target_brightness_iso: 89.0,
    };
    let encoded = encode_with_checksum(&seq).expect("encode bleaching sequence failed");
    let (decoded, consumed): (BleachingSequence, _) =
        decode_with_checksum(&encoded).expect("decode bleaching sequence failed");
    assert_eq!(decoded, seq, "bleaching sequence mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: Paper machine speed and tension readings
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PaperMachineSpeedTension {
    section_name: String,
    machine_speed_m_per_min: f64,
    sheet_tension_n_per_m: f64,
    draw_pct: f64,
    motor_load_kw: f64,
    roll_diameter_mm: f64,
}

#[test]
fn test_paper_machine_speed_tension() {
    let readings = vec![
        PaperMachineSpeedTension {
            section_name: "Forming Section".to_string(),
            machine_speed_m_per_min: 1200.0,
            sheet_tension_n_per_m: 350.0,
            draw_pct: 0.0,
            motor_load_kw: 450.0,
            roll_diameter_mm: 1500.0,
        },
        PaperMachineSpeedTension {
            section_name: "Press Section".to_string(),
            machine_speed_m_per_min: 1205.0,
            sheet_tension_n_per_m: 600.0,
            draw_pct: 0.42,
            motor_load_kw: 820.0,
            roll_diameter_mm: 1200.0,
        },
        PaperMachineSpeedTension {
            section_name: "Dryer Section".to_string(),
            machine_speed_m_per_min: 1210.0,
            sheet_tension_n_per_m: 500.0,
            draw_pct: 0.41,
            motor_load_kw: 1100.0,
            roll_diameter_mm: 1830.0,
        },
    ];
    let encoded = encode_with_checksum(&readings).expect("encode speed tension readings failed");
    let (decoded, consumed): (Vec<PaperMachineSpeedTension>, _) =
        decode_with_checksum(&encoded).expect("decode speed tension readings failed");
    assert_eq!(decoded, readings, "speed tension readings mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 5: Caliper thickness measurement
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct CaliperMeasurement {
    measurement_id: u64,
    position_cd_mm: f64,
    caliper_micrometers: f64,
    target_micrometers: f64,
    deviation_micrometers: f64,
    within_tolerance: bool,
}

#[test]
fn test_caliper_thickness_measurement() {
    let measurements = vec![
        CaliperMeasurement {
            measurement_id: 100001,
            position_cd_mm: 250.0,
            caliper_micrometers: 105.2,
            target_micrometers: 105.0,
            deviation_micrometers: 0.2,
            within_tolerance: true,
        },
        CaliperMeasurement {
            measurement_id: 100002,
            position_cd_mm: 1250.0,
            caliper_micrometers: 108.7,
            target_micrometers: 105.0,
            deviation_micrometers: 3.7,
            within_tolerance: false,
        },
    ];
    let encoded = encode_with_checksum(&measurements).expect("encode caliper measurements failed");
    let (decoded, consumed): (Vec<CaliperMeasurement>, _) =
        decode_with_checksum(&encoded).expect("decode caliper measurements failed");
    assert_eq!(decoded, measurements, "caliper measurements mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: Brightness and opacity testing
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct BrightnessOpacityTest {
    sample_id: String,
    iso_brightness: f64,
    tappi_brightness: f64,
    opacity_pct: f64,
    cielab_l_star: f64,
    cielab_a_star: f64,
    cielab_b_star: f64,
    fluorescence_contribution: f64,
}

#[test]
fn test_brightness_opacity() {
    let result = BrightnessOpacityTest {
        sample_id: "BRT-2026-03-15-001".to_string(),
        iso_brightness: 89.2,
        tappi_brightness: 87.8,
        opacity_pct: 92.5,
        cielab_l_star: 96.1,
        cielab_a_star: 0.3,
        cielab_b_star: -1.2,
        fluorescence_contribution: 1.4,
    };
    let encoded = encode_with_checksum(&result).expect("encode brightness opacity test failed");
    let (decoded, consumed): (BrightnessOpacityTest, _) =
        decode_with_checksum(&encoded).expect("decode brightness opacity test failed");
    assert_eq!(decoded, result, "brightness opacity test mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: Stock preparation consistency
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum StockPrepStage {
    HighDensityCleaner,
    LowDensityCleaner,
    Deflaker,
    Refiner,
    MixingChest,
    MachineChest,
    SiloTank,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StockConsistencyReading {
    stage: StockPrepStage,
    consistency_pct: f64,
    flow_rate_liters_per_min: f64,
    freeness_csf_ml: u32,
    temperature_c: f64,
}

#[test]
fn test_stock_preparation_consistency() {
    let readings = vec![
        StockConsistencyReading {
            stage: StockPrepStage::HighDensityCleaner,
            consistency_pct: 3.5,
            flow_rate_liters_per_min: 8500.0,
            freeness_csf_ml: 520,
            temperature_c: 48.0,
        },
        StockConsistencyReading {
            stage: StockPrepStage::Refiner,
            consistency_pct: 4.0,
            flow_rate_liters_per_min: 7200.0,
            freeness_csf_ml: 380,
            temperature_c: 52.0,
        },
        StockConsistencyReading {
            stage: StockPrepStage::MachineChest,
            consistency_pct: 3.2,
            flow_rate_liters_per_min: 9100.0,
            freeness_csf_ml: 370,
            temperature_c: 45.0,
        },
    ];
    let encoded =
        encode_with_checksum(&readings).expect("encode stock consistency readings failed");
    let (decoded, consumed): (Vec<StockConsistencyReading>, _) =
        decode_with_checksum(&encoded).expect("decode stock consistency readings failed");
    assert_eq!(decoded, readings, "stock consistency readings mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 8: Headbox dilution control
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct HeadboxDilutionControl {
    zone_index: u16,
    slice_opening_mm: f64,
    dilution_valve_pct: f64,
    local_consistency_pct: f64,
    jet_to_wire_ratio: f64,
    total_head_kpa: f64,
    lip_geometry_angle_deg: f64,
}

#[test]
fn test_headbox_dilution_control() {
    let zones: Vec<HeadboxDilutionControl> = (0..5)
        .map(|i| HeadboxDilutionControl {
            zone_index: i,
            slice_opening_mm: 8.0 + (i as f64) * 0.05,
            dilution_valve_pct: 45.0 + (i as f64) * 2.0,
            local_consistency_pct: 0.85 + (i as f64) * 0.01,
            jet_to_wire_ratio: 0.98,
            total_head_kpa: 42.5,
            lip_geometry_angle_deg: 3.5,
        })
        .collect();
    let encoded = encode_with_checksum(&zones).expect("encode headbox dilution controls failed");
    let (decoded, consumed): (Vec<HeadboxDilutionControl>, _) =
        decode_with_checksum(&encoded).expect("decode headbox dilution controls failed");
    assert_eq!(decoded, zones, "headbox dilution controls mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 9: Press section nip pressures
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum PressType {
    StraightThrough,
    SuctionPress,
    ShoePress,
    SmoothPress,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PressNipPressure {
    press_position: u8,
    press_type: PressType,
    linear_load_kn_per_m: f64,
    nip_width_mm: f64,
    peak_pressure_mpa: f64,
    felt_conditioning_vacuum_kpa: f64,
    sheet_dryness_in_pct: f64,
    sheet_dryness_out_pct: f64,
}

#[test]
fn test_press_section_nip_pressures() {
    let nips = vec![
        PressNipPressure {
            press_position: 1,
            press_type: PressType::ShoePress,
            linear_load_kn_per_m: 1000.0,
            nip_width_mm: 250.0,
            peak_pressure_mpa: 4.0,
            felt_conditioning_vacuum_kpa: 35.0,
            sheet_dryness_in_pct: 20.0,
            sheet_dryness_out_pct: 42.0,
        },
        PressNipPressure {
            press_position: 2,
            press_type: PressType::SmoothPress,
            linear_load_kn_per_m: 80.0,
            nip_width_mm: 35.0,
            peak_pressure_mpa: 2.3,
            felt_conditioning_vacuum_kpa: 25.0,
            sheet_dryness_in_pct: 42.0,
            sheet_dryness_out_pct: 48.0,
        },
    ];
    let encoded = encode_with_checksum(&nips).expect("encode press nip pressures failed");
    let (decoded, consumed): (Vec<PressNipPressure>, _) =
        decode_with_checksum(&encoded).expect("decode press nip pressures failed");
    assert_eq!(decoded, nips, "press nip pressures mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 10: Coating formulation
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct CoatingIngredient {
    name: String,
    parts_per_hundred: f64,
    solids_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CoatingFormulation {
    formulation_id: String,
    target_coat_weight_gsm: f64,
    target_solids_pct: f64,
    viscosity_mpa_s: f64,
    ph: f64,
    ingredients: Vec<CoatingIngredient>,
}

#[test]
fn test_coating_formulation() {
    let formula = CoatingFormulation {
        formulation_id: "LWC-TOP-2026A".to_string(),
        target_coat_weight_gsm: 10.0,
        target_solids_pct: 64.0,
        viscosity_mpa_s: 1200.0,
        ph: 8.5,
        ingredients: vec![
            CoatingIngredient {
                name: "Ground Calcium Carbonate".to_string(),
                parts_per_hundred: 70.0,
                solids_pct: 75.0,
            },
            CoatingIngredient {
                name: "Fine Clay".to_string(),
                parts_per_hundred: 30.0,
                solids_pct: 70.0,
            },
            CoatingIngredient {
                name: "Styrene-Butadiene Latex".to_string(),
                parts_per_hundred: 12.0,
                solids_pct: 50.0,
            },
            CoatingIngredient {
                name: "CMC Thickener".to_string(),
                parts_per_hundred: 0.3,
                solids_pct: 5.0,
            },
        ],
    };
    let encoded = encode_with_checksum(&formula).expect("encode coating formulation failed");
    let (decoded, consumed): (CoatingFormulation, _) =
        decode_with_checksum(&encoded).expect("decode coating formulation failed");
    assert_eq!(decoded, formula, "coating formulation mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 11: Winder trim specifications
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct WinderTrimSpec {
    set_number: u32,
    reel_width_mm: f64,
    trim_width_front_mm: f64,
    trim_width_back_mm: f64,
    core_diameter_mm: f64,
    target_roll_diameter_mm: f64,
    winding_tension_n_per_m: f64,
    nip_load_n_per_m: f64,
    target_roll_hardness: u32,
}

#[test]
fn test_winder_trim_specifications() {
    let spec = WinderTrimSpec {
        set_number: 42,
        reel_width_mm: 8600.0,
        trim_width_front_mm: 15.0,
        trim_width_back_mm: 15.0,
        core_diameter_mm: 76.2,
        target_roll_diameter_mm: 1200.0,
        winding_tension_n_per_m: 400.0,
        nip_load_n_per_m: 250.0,
        target_roll_hardness: 78,
    };
    let encoded = encode_with_checksum(&spec).expect("encode winder trim spec failed");
    let (decoded, consumed): (WinderTrimSpec, _) =
        decode_with_checksum(&encoded).expect("decode winder trim spec failed");
    assert_eq!(decoded, spec, "winder trim spec mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 12: Broke handling state machine
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum BrokeHandlingState {
    NoBroke,
    WebBreakDetected {
        dryer_group: u8,
    },
    CouchPitDiverting,
    BrokePulping {
        pulper_level_pct: f64,
    },
    BrokeChestStorage {
        chest_level_pct: f64,
        consistency_pct: f64,
    },
    BrokeBeingReintroduced {
        dosing_rate_pct: f64,
    },
    RecoveryComplete,
}

#[test]
fn test_broke_handling_states() {
    let states = vec![
        BrokeHandlingState::NoBroke,
        BrokeHandlingState::WebBreakDetected { dryer_group: 3 },
        BrokeHandlingState::CouchPitDiverting,
        BrokeHandlingState::BrokePulping {
            pulper_level_pct: 65.0,
        },
        BrokeHandlingState::BrokeChestStorage {
            chest_level_pct: 78.0,
            consistency_pct: 3.8,
        },
        BrokeHandlingState::BrokeBeingReintroduced {
            dosing_rate_pct: 15.0,
        },
        BrokeHandlingState::RecoveryComplete,
    ];
    let encoded = encode_with_checksum(&states).expect("encode broke handling states failed");
    let (decoded, consumed): (Vec<BrokeHandlingState>, _) =
        decode_with_checksum(&encoded).expect("decode broke handling states failed");
    assert_eq!(decoded, states, "broke handling states mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 13: Chemical recovery boiler data
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecoveryBoilerData {
    boiler_id: String,
    black_liquor_solids_pct: f64,
    firing_rate_tons_ds_per_day: f64,
    steam_pressure_mpa: f64,
    steam_temperature_c: f64,
    steam_production_tons_per_hr: f64,
    flue_gas_temperature_c: f64,
    smelt_flow_rate_kg_per_min: f64,
    reduction_efficiency_pct: f64,
    so2_emissions_ppm: f64,
    nox_emissions_ppm: f64,
}

#[test]
fn test_chemical_recovery_boiler_data() {
    let data = RecoveryBoilerData {
        boiler_id: "RB-01".to_string(),
        black_liquor_solids_pct: 72.5,
        firing_rate_tons_ds_per_day: 3200.0,
        steam_pressure_mpa: 8.5,
        steam_temperature_c: 480.0,
        steam_production_tons_per_hr: 350.0,
        flue_gas_temperature_c: 165.0,
        smelt_flow_rate_kg_per_min: 180.0,
        reduction_efficiency_pct: 95.5,
        so2_emissions_ppm: 12.0,
        nox_emissions_ppm: 85.0,
    };
    let encoded = encode_with_checksum(&data).expect("encode recovery boiler data failed");
    let (decoded, consumed): (RecoveryBoilerData, _) =
        decode_with_checksum(&encoded).expect("decode recovery boiler data failed");
    assert_eq!(decoded, data, "recovery boiler data mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 14: Pulp washing efficiency
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PulpWashingStage {
    washer_type: String,
    displacement_ratio: f64,
    dilution_factor: f64,
    inlet_cod_mg_per_l: f64,
    outlet_cod_mg_per_l: f64,
    norden_efficiency_pct: f64,
    mat_consistency_pct: f64,
    discharge_consistency_pct: f64,
}

#[test]
fn test_pulp_washing_efficiency() {
    let stages = vec![
        PulpWashingStage {
            washer_type: "Drum Displacer DD-5500".to_string(),
            displacement_ratio: 0.85,
            dilution_factor: 2.5,
            inlet_cod_mg_per_l: 45000.0,
            outlet_cod_mg_per_l: 6500.0,
            norden_efficiency_pct: 85.6,
            mat_consistency_pct: 12.0,
            discharge_consistency_pct: 30.0,
        },
        PulpWashingStage {
            washer_type: "Press Washer PW-800".to_string(),
            displacement_ratio: 0.78,
            dilution_factor: 2.0,
            inlet_cod_mg_per_l: 6500.0,
            outlet_cod_mg_per_l: 1400.0,
            norden_efficiency_pct: 78.5,
            mat_consistency_pct: 10.0,
            discharge_consistency_pct: 32.0,
        },
    ];
    let encoded = encode_with_checksum(&stages).expect("encode pulp washing stages failed");
    let (decoded, consumed): (Vec<PulpWashingStage>, _) =
        decode_with_checksum(&encoded).expect("decode pulp washing stages failed");
    assert_eq!(decoded, stages, "pulp washing stages mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Lime kiln operation parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct LimeKilnOperation {
    kiln_id: String,
    feed_rate_tons_per_hr: f64,
    burning_zone_temp_c: f64,
    back_end_temp_c: f64,
    exit_gas_temp_c: f64,
    fuel_consumption_gj_per_hr: f64,
    residual_carbonate_pct: f64,
    availability_pct: f64,
    ring_formation_index: u8,
}

#[test]
fn test_lime_kiln_operation() {
    let kiln = LimeKilnOperation {
        kiln_id: "LK-02".to_string(),
        feed_rate_tons_per_hr: 18.5,
        burning_zone_temp_c: 1250.0,
        back_end_temp_c: 350.0,
        exit_gas_temp_c: 220.0,
        fuel_consumption_gj_per_hr: 55.0,
        residual_carbonate_pct: 1.8,
        availability_pct: 97.2,
        ring_formation_index: 2,
    };
    let encoded = encode_with_checksum(&kiln).expect("encode lime kiln operation failed");
    let (decoded, consumed): (LimeKilnOperation, _) =
        decode_with_checksum(&encoded).expect("decode lime kiln operation failed");
    assert_eq!(decoded, kiln, "lime kiln operation mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: Dryer section steam and condensate
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct DryerGroupSteam {
    group_number: u8,
    num_cylinders: u8,
    steam_pressure_kpa: f64,
    condensate_level_pct: f64,
    differential_pressure_kpa: f64,
    surface_temperature_c: f64,
    sheet_temperature_entering_c: f64,
    sheet_temperature_leaving_c: f64,
    siphon_type: String,
}

#[test]
fn test_dryer_section_steam_condensate() {
    let groups = vec![
        DryerGroupSteam {
            group_number: 1,
            num_cylinders: 6,
            steam_pressure_kpa: 120.0,
            condensate_level_pct: 25.0,
            differential_pressure_kpa: 20.0,
            surface_temperature_c: 85.0,
            sheet_temperature_entering_c: 48.0,
            sheet_temperature_leaving_c: 65.0,
            siphon_type: "Stationary".to_string(),
        },
        DryerGroupSteam {
            group_number: 5,
            num_cylinders: 8,
            steam_pressure_kpa: 450.0,
            condensate_level_pct: 18.0,
            differential_pressure_kpa: 35.0,
            surface_temperature_c: 148.0,
            sheet_temperature_entering_c: 92.0,
            sheet_temperature_leaving_c: 110.0,
            siphon_type: "Rotary".to_string(),
        },
    ];
    let encoded = encode_with_checksum(&groups).expect("encode dryer group steam data failed");
    let (decoded, consumed): (Vec<DryerGroupSteam>, _) =
        decode_with_checksum(&encoded).expect("decode dryer group steam data failed");
    assert_eq!(decoded, groups, "dryer group steam data mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 17: Evaporator train effect
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct EvaporatorEffect {
    effect_number: u8,
    liquor_solids_in_pct: f64,
    liquor_solids_out_pct: f64,
    boiling_point_rise_c: f64,
    heat_transfer_area_m2: f64,
    overall_htc_w_per_m2_k: f64,
    vacuum_kpa: f64,
    scaling_index: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EvaporatorTrain {
    train_id: String,
    num_effects: u8,
    steam_economy: f64,
    effects: Vec<EvaporatorEffect>,
}

#[test]
fn test_evaporator_train() {
    let train = EvaporatorTrain {
        train_id: "EVAP-A".to_string(),
        num_effects: 6,
        steam_economy: 5.2,
        effects: vec![
            EvaporatorEffect {
                effect_number: 1,
                liquor_solids_in_pct: 15.0,
                liquor_solids_out_pct: 22.0,
                boiling_point_rise_c: 1.5,
                heat_transfer_area_m2: 850.0,
                overall_htc_w_per_m2_k: 2500.0,
                vacuum_kpa: 0.0,
                scaling_index: 0.3,
            },
            EvaporatorEffect {
                effect_number: 6,
                liquor_solids_in_pct: 55.0,
                liquor_solids_out_pct: 72.0,
                boiling_point_rise_c: 18.0,
                heat_transfer_area_m2: 1200.0,
                overall_htc_w_per_m2_k: 800.0,
                vacuum_kpa: 65.0,
                scaling_index: 2.1,
            },
        ],
    };
    let encoded = encode_with_checksum(&train).expect("encode evaporator train failed");
    let (decoded, consumed): (EvaporatorTrain, _) =
        decode_with_checksum(&encoded).expect("decode evaporator train failed");
    assert_eq!(decoded, train, "evaporator train mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 18: Fiber furnish composition
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct FurnishComponent {
    pulp_type: String,
    proportion_pct: f64,
    freeness_csf_ml: u32,
    fiber_length_mm: f64,
    coarseness_mg_per_m: f64,
    fines_content_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FiberFurnish {
    grade_name: String,
    basis_weight_gsm: f64,
    components: Vec<FurnishComponent>,
}

#[test]
fn test_fiber_furnish_composition() {
    let furnish = FiberFurnish {
        grade_name: "Copy Paper 80gsm".to_string(),
        basis_weight_gsm: 80.0,
        components: vec![
            FurnishComponent {
                pulp_type: "Hardwood BKP".to_string(),
                proportion_pct: 65.0,
                freeness_csf_ml: 420,
                fiber_length_mm: 0.85,
                coarseness_mg_per_m: 0.065,
                fines_content_pct: 12.0,
            },
            FurnishComponent {
                pulp_type: "Softwood BKP".to_string(),
                proportion_pct: 25.0,
                freeness_csf_ml: 550,
                fiber_length_mm: 2.4,
                coarseness_mg_per_m: 0.18,
                fines_content_pct: 8.0,
            },
            FurnishComponent {
                pulp_type: "DIP (Deinked Pulp)".to_string(),
                proportion_pct: 10.0,
                freeness_csf_ml: 320,
                fiber_length_mm: 1.1,
                coarseness_mg_per_m: 0.12,
                fines_content_pct: 18.0,
            },
        ],
    };
    let encoded = encode_with_checksum(&furnish).expect("encode fiber furnish failed");
    let (decoded, consumed): (FiberFurnish, _) =
        decode_with_checksum(&encoded).expect("decode fiber furnish failed");
    assert_eq!(decoded, furnish, "fiber furnish mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 19: Calendering parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum CalenderType {
    SoftNip,
    HardNip,
    Supercalender,
    MultinipCalender,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CalenderingParams {
    calender_type: CalenderType,
    num_nips: u8,
    linear_load_kn_per_m: f64,
    hot_roll_temperature_c: f64,
    speed_m_per_min: f64,
    gloss_before: f64,
    gloss_after: f64,
    smoothness_pps_micrometers: f64,
    caliper_reduction_pct: f64,
}

#[test]
fn test_calendering_parameters() {
    let params = CalenderingParams {
        calender_type: CalenderType::SoftNip,
        num_nips: 2,
        linear_load_kn_per_m: 250.0,
        hot_roll_temperature_c: 220.0,
        speed_m_per_min: 1200.0,
        gloss_before: 35.0,
        gloss_after: 55.0,
        smoothness_pps_micrometers: 1.8,
        caliper_reduction_pct: 8.5,
    };
    let encoded = encode_with_checksum(&params).expect("encode calendering params failed");
    let (decoded, consumed): (CalenderingParams, _) =
        decode_with_checksum(&encoded).expect("decode calendering params failed");
    assert_eq!(decoded, params, "calendering params mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 20: Retention and drainage aid dosing
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum ChemicalType {
    Cpam,
    MicroparticleSilica,
    Bentonite,
    Starch,
    Pei,
    Alum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RetentionAidDosing {
    chemical_type: ChemicalType,
    addition_point: String,
    dose_g_per_ton: f64,
    concentration_pct: f64,
    flow_rate_ml_per_min: f64,
    first_pass_retention_pct: f64,
    ash_retention_pct: f64,
}

#[test]
fn test_retention_drainage_aid_dosing() {
    let dosing = vec![
        RetentionAidDosing {
            chemical_type: ChemicalType::Cpam,
            addition_point: "Before Screen".to_string(),
            dose_g_per_ton: 250.0,
            concentration_pct: 0.1,
            flow_rate_ml_per_min: 420.0,
            first_pass_retention_pct: 88.0,
            ash_retention_pct: 72.0,
        },
        RetentionAidDosing {
            chemical_type: ChemicalType::MicroparticleSilica,
            addition_point: "After Screen".to_string(),
            dose_g_per_ton: 800.0,
            concentration_pct: 3.5,
            flow_rate_ml_per_min: 180.0,
            first_pass_retention_pct: 92.0,
            ash_retention_pct: 80.0,
        },
    ];
    let encoded = encode_with_checksum(&dosing).expect("encode retention aid dosing failed");
    let (decoded, consumed): (Vec<RetentionAidDosing>, _) =
        decode_with_checksum(&encoded).expect("decode retention aid dosing failed");
    assert_eq!(decoded, dosing, "retention aid dosing mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: Effluent treatment parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct EffluentTreatmentData {
    treatment_stage: String,
    flow_rate_m3_per_hr: f64,
    bod5_mg_per_l: f64,
    cod_mg_per_l: f64,
    tss_mg_per_l: f64,
    aox_kg_per_ton: f64,
    ph: f64,
    temperature_c: f64,
    color_units: u32,
    dissolved_oxygen_mg_per_l: f64,
}

#[test]
fn test_effluent_treatment_parameters() {
    let stages = vec![
        EffluentTreatmentData {
            treatment_stage: "Primary Clarifier".to_string(),
            flow_rate_m3_per_hr: 2800.0,
            bod5_mg_per_l: 350.0,
            cod_mg_per_l: 1200.0,
            tss_mg_per_l: 800.0,
            aox_kg_per_ton: 0.3,
            ph: 7.2,
            temperature_c: 38.0,
            color_units: 2500,
            dissolved_oxygen_mg_per_l: 0.5,
        },
        EffluentTreatmentData {
            treatment_stage: "Aerated Stabilization Basin".to_string(),
            flow_rate_m3_per_hr: 2800.0,
            bod5_mg_per_l: 25.0,
            cod_mg_per_l: 280.0,
            tss_mg_per_l: 45.0,
            aox_kg_per_ton: 0.15,
            ph: 7.8,
            temperature_c: 32.0,
            color_units: 800,
            dissolved_oxygen_mg_per_l: 4.5,
        },
    ];
    let encoded = encode_with_checksum(&stages).expect("encode effluent treatment data failed");
    let (decoded, consumed): (Vec<EffluentTreatmentData>, _) =
        decode_with_checksum(&encoded).expect("decode effluent treatment data failed");
    assert_eq!(decoded, stages, "effluent treatment data mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}

// ---------------------------------------------------------------------------
// Test 22: Finished reel quality report
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum QualityDisposition {
    Prime,
    SecondGrade { reason: String },
    Broke,
    HoldForReview,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FinishedReelReport {
    reel_number: u64,
    grade_code: String,
    basis_weight_gsm: f64,
    caliper_micrometers: f64,
    moisture_pct: f64,
    brightness_iso: f64,
    opacity_pct: f64,
    smoothness_pps_micrometers: f64,
    tensile_md_kn_per_m: f64,
    tensile_cd_kn_per_m: f64,
    tear_md_mn: f64,
    tear_cd_mn: f64,
    reel_weight_kg: f64,
    reel_width_mm: f64,
    disposition: QualityDisposition,
}

#[test]
fn test_finished_reel_quality_report() {
    let report = FinishedReelReport {
        reel_number: 20260315_0042,
        grade_code: "WFC-80".to_string(),
        basis_weight_gsm: 80.2,
        caliper_micrometers: 104.5,
        moisture_pct: 4.8,
        brightness_iso: 89.1,
        opacity_pct: 92.8,
        smoothness_pps_micrometers: 1.6,
        tensile_md_kn_per_m: 5.2,
        tensile_cd_kn_per_m: 2.1,
        tear_md_mn: 380.0,
        tear_cd_mn: 520.0,
        reel_weight_kg: 28500.0,
        reel_width_mm: 8600.0,
        disposition: QualityDisposition::Prime,
    };
    let encoded = encode_with_checksum(&report).expect("encode finished reel report failed");
    let (decoded, consumed): (FinishedReelReport, _) =
        decode_with_checksum(&encoded).expect("decode finished reel report failed");
    assert_eq!(decoded, report, "finished reel report mismatch");
    assert_eq!(consumed, encoded.len(), "consumed bytes mismatch");
}
