//! Advanced complex enum encoding tests for OxiCode - set 37
//! Theme: Oil and gas upstream operations and drilling domain types.
//! 22 test functions covering well types, drilling mud, BOP stacks, formation
//! evaluation logs, production separators, pipeline pigging, flare systems,
//! HSE incidents, well interventions, reservoir simulation, and artificial lift.

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

// --- Well classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum WellType {
    Exploration {
        prospect_name: String,
        target_depth_ft: u32,
        spud_date_epoch: u64,
    },
    Appraisal {
        discovery_well_id: String,
        test_zones: Vec<String>,
    },
    Development {
        pad_name: String,
        slot_number: u8,
        is_horizontal: bool,
    },
    Injection {
        injected_fluid: InjectedFluid,
        max_pressure_psi: u32,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum InjectedFluid {
    Water,
    Gas,
    Steam,
    Co2,
    Polymer(String),
}

// --- Drilling mud ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum DrillingMudType {
    WaterBased {
        density_ppg: u16,
        bentonite_pct: u8,
        polymer_additive: Option<String>,
    },
    OilBased {
        density_ppg: u16,
        oil_water_ratio: (u8, u8),
        emulsifier: String,
    },
    SyntheticBased {
        density_ppg: u16,
        base_fluid: String,
        toxicity_class: u8,
    },
    FoamBased {
        quality_pct: u8,
        surfactant: String,
    },
}

// --- BOP stack ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum BopRamType {
    Blind,
    Pipe,
    ShearBlind,
    Variable,
    Casing(u16),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BopStack {
    stack_id: String,
    pressure_rating_psi: u32,
    annular_preventers: u8,
    rams: Vec<BopRamType>,
    choke_line_id_inches: f32,
    kill_line_id_inches: f32,
    last_test_epoch: u64,
}

// --- Formation evaluation logs ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum FormationLog {
    GammaRay {
        depth_ft: f64,
        api_units: f64,
        shale_baseline: f64,
        sand_baseline: f64,
    },
    Resistivity {
        depth_ft: f64,
        shallow_ohm_m: f64,
        medium_ohm_m: f64,
        deep_ohm_m: f64,
    },
    Sonic {
        depth_ft: f64,
        delta_t_us_per_ft: f64,
        compressional: f64,
        shear: Option<f64>,
    },
    NeutronDensity {
        depth_ft: f64,
        neutron_porosity_pu: f64,
        bulk_density_gcc: f64,
    },
    NMR {
        depth_ft: f64,
        t2_distribution: Vec<f64>,
        bound_fluid_pct: f64,
        free_fluid_pct: f64,
    },
}

// --- Production separator ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SeparatorState {
    Idle,
    StartingUp {
        target_pressure_psi: u32,
        warmup_seconds: u32,
    },
    Operating {
        oil_rate_bpd: f64,
        gas_rate_mscfd: f64,
        water_rate_bpd: f64,
        pressure_psi: f64,
        temperature_f: f64,
    },
    ShuttingDown {
        reason: String,
        drain_remaining_bbl: f64,
    },
    Emergency(String),
}

// --- Pipeline pigging ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum PigType {
    Utility,
    Cleaning {
        brush_material: String,
    },
    Gauging {
        min_bore_inches: f32,
    },
    SmartMfl {
        sensor_count: u32,
        resolution_mm: f32,
    },
    SmartUt {
        wall_thickness_min_mm: f32,
    },
    Batching,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PiggingRun {
    run_id: u64,
    pipeline_name: String,
    pig: PigType,
    launch_pressure_psi: u32,
    receive_pressure_psi: u32,
    distance_miles: f64,
    anomalies_found: Vec<PipelineAnomaly>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PipelineAnomaly {
    MetalLoss {
        clock_position: u8,
        depth_pct: f64,
        length_inches: f64,
    },
    Dent {
        depth_pct: f64,
        associated_metal_loss: bool,
    },
    Crack {
        orientation: CrackOrientation,
        length_inches: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CrackOrientation {
    Axial,
    Circumferential,
    Oblique(f64),
}

// --- Flare system ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum FlareMode {
    PilotOnly,
    LowRate {
        flow_mscfd: f64,
    },
    HighRate {
        flow_mscfd: f64,
        smoke_suppression_active: bool,
    },
    Emergency {
        flow_mscfd: f64,
        source_psv_ids: Vec<String>,
    },
    Purging {
        purge_gas: String,
    },
    Shutdown,
}

// --- HSE incident ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum HseIncidentClass {
    NearMiss {
        description: String,
    },
    FirstAid {
        injury_type: String,
    },
    MedicalTreatment {
        injury_type: String,
        days_restricted: u16,
    },
    LostTimeInjury {
        injury_type: String,
        days_lost: u16,
    },
    EnvironmentalRelease {
        substance: String,
        volume_bbl: f64,
        contained: bool,
    },
    ProcessSafetyEvent {
        tier: u8,
        release_type: String,
        overpressure_psi: Option<f64>,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HseReport {
    report_id: u64,
    location: String,
    timestamp_epoch: u64,
    incident: HseIncidentClass,
    corrective_actions: Vec<String>,
}

// --- Well intervention ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum WellIntervention {
    Wireline {
        tool_string: Vec<String>,
        max_depth_ft: u32,
    },
    CoiledTubing {
        od_inches: f32,
        acid_volume_gal: u32,
    },
    Snubbing {
        pipe_od_inches: f32,
        wellhead_pressure_psi: u32,
    },
    HydraulicWorkover {
        rig_capacity_tons: u32,
    },
    SlicklineKickover {
        target_zone: String,
    },
    PlugAndAbandon {
        cement_plugs: Vec<CementPlug>,
        surface_restoration: bool,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CementPlug {
    top_depth_ft: u32,
    bottom_depth_ft: u32,
    cement_class: String,
}

// --- Reservoir simulation cell ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ReservoirCellPhase {
    SinglePhaseOil,
    SinglePhaseGas,
    SinglePhaseWater,
    TwoPhaseOilWater {
        water_saturation: f64,
    },
    TwoPhaseOilGas {
        gas_saturation: f64,
    },
    ThreePhase {
        oil_saturation: f64,
        water_saturation: f64,
        gas_saturation: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReservoirCell {
    i: u32,
    j: u32,
    k: u32,
    porosity: f64,
    permeability_md: f64,
    pressure_psi: f64,
    temperature_f: f64,
    phase: ReservoirCellPhase,
}

// --- Artificial lift ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ArtificialLiftMethod {
    Esp {
        pump_stages: u16,
        frequency_hz: f64,
        intake_pressure_psi: f64,
        motor_amps: f64,
        motor_temp_f: f64,
    },
    RodPump {
        stroke_length_inches: f64,
        strokes_per_min: f64,
        pump_fillage_pct: f64,
        rod_grade: String,
    },
    GasLift {
        injection_rate_mscfd: f64,
        injection_pressure_psi: f64,
        valve_depths_ft: Vec<u32>,
        operating_valve_index: u8,
    },
    PlungerLift {
        cycle_time_min: u32,
        arrival_velocity_fpm: f64,
    },
    Pcp {
        rotor_speed_rpm: f64,
        elastomer_type: String,
        differential_pressure_psi: f64,
    },
    JetPump {
        nozzle_size: String,
        throat_size: String,
        power_fluid_rate_bpd: f64,
    },
}

// --- Drill string component ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum DrillStringComponent {
    DrillPipe {
        od_inches: f32,
        weight_ppf: f32,
        grade: String,
    },
    HeavyWeight {
        od_inches: f32,
        length_ft: f32,
    },
    DrillCollar {
        od_inches: f32,
        id_inches: f32,
        length_ft: f32,
    },
    Stabilizer {
        blade_od_inches: f32,
        blade_count: u8,
    },
    Mwd {
        gamma_enabled: bool,
        survey_interval_ft: u32,
    },
    Lwd {
        tools: Vec<String>,
    },
    Motor {
        bend_angle_deg: f64,
        flow_range_gpm: (u32, u32),
    },
    RssTool {
        tool_face_mode: String,
        build_rate_deg_per_100ft: f64,
    },
}

// ==========================================================================
// Tests
// ==========================================================================

// --- Test 1: Exploration well roundtrip ---
#[test]
fn test_exploration_well_roundtrip() {
    let val = WellType::Exploration {
        prospect_name: "Orion Deep".to_string(),
        target_depth_ft: 18500,
        spud_date_epoch: 1_700_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode Exploration well");
    let (decoded, _): (WellType, usize) =
        decode_from_slice(&bytes).expect("decode Exploration well");
    assert_eq!(val, decoded);
}

// --- Test 2: Injection well with CO2 fluid ---
#[test]
fn test_injection_well_co2_roundtrip() {
    let val = WellType::Injection {
        injected_fluid: InjectedFluid::Co2,
        max_pressure_psi: 6200,
    };
    let bytes = encode_to_vec(&val).expect("encode Injection CO2 well");
    let (decoded, _): (WellType, usize) =
        decode_from_slice(&bytes).expect("decode Injection CO2 well");
    assert_eq!(val, decoded);
}

// --- Test 3: Water-based drilling mud ---
#[test]
fn test_water_based_mud_roundtrip() {
    let val = DrillingMudType::WaterBased {
        density_ppg: 112,
        bentonite_pct: 6,
        polymer_additive: Some("PHPA".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode WBM");
    let (decoded, _): (DrillingMudType, usize) = decode_from_slice(&bytes).expect("decode WBM");
    assert_eq!(val, decoded);
}

// --- Test 4: Oil-based drilling mud ---
#[test]
fn test_oil_based_mud_roundtrip() {
    let val = DrillingMudType::OilBased {
        density_ppg: 145,
        oil_water_ratio: (80, 20),
        emulsifier: "Primary tall-oil amine".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode OBM");
    let (decoded, _): (DrillingMudType, usize) = decode_from_slice(&bytes).expect("decode OBM");
    assert_eq!(val, decoded);
}

// --- Test 5: BOP stack with multiple ram types ---
#[test]
fn test_bop_stack_roundtrip() {
    let val = BopStack {
        stack_id: "BOP-GOM-2024-017".to_string(),
        pressure_rating_psi: 15000,
        annular_preventers: 2,
        rams: vec![
            BopRamType::ShearBlind,
            BopRamType::Pipe,
            BopRamType::Variable,
            BopRamType::Casing(9),
        ],
        choke_line_id_inches: 3.0,
        kill_line_id_inches: 3.0,
        last_test_epoch: 1_710_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode BOP stack");
    let (decoded, _): (BopStack, usize) = decode_from_slice(&bytes).expect("decode BOP stack");
    assert_eq!(val, decoded);
}

// --- Test 6: Gamma ray formation log ---
#[test]
fn test_gamma_ray_log_roundtrip() {
    let val = FormationLog::GammaRay {
        depth_ft: 12450.5,
        api_units: 85.3,
        shale_baseline: 130.0,
        sand_baseline: 20.0,
    };
    let bytes = encode_to_vec(&val).expect("encode GR log");
    let (decoded, _): (FormationLog, usize) = decode_from_slice(&bytes).expect("decode GR log");
    assert_eq!(val, decoded);
}

// --- Test 7: Deep resistivity log ---
#[test]
fn test_resistivity_log_roundtrip() {
    let val = FormationLog::Resistivity {
        depth_ft: 14200.0,
        shallow_ohm_m: 1.2,
        medium_ohm_m: 5.6,
        deep_ohm_m: 45.0,
    };
    let bytes = encode_to_vec(&val).expect("encode resistivity log");
    let (decoded, _): (FormationLog, usize) =
        decode_from_slice(&bytes).expect("decode resistivity log");
    assert_eq!(val, decoded);
}

// --- Test 8: Sonic log with optional shear ---
#[test]
fn test_sonic_log_with_shear_roundtrip() {
    let val = FormationLog::Sonic {
        depth_ft: 15300.0,
        delta_t_us_per_ft: 68.5,
        compressional: 14600.0,
        shear: Some(7800.0),
    };
    let bytes = encode_to_vec(&val).expect("encode sonic log");
    let (decoded, _): (FormationLog, usize) = decode_from_slice(&bytes).expect("decode sonic log");
    assert_eq!(val, decoded);
}

// --- Test 9: NMR log with T2 distribution ---
#[test]
fn test_nmr_log_roundtrip() {
    let val = FormationLog::NMR {
        depth_ft: 13100.0,
        t2_distribution: vec![0.3, 1.0, 3.0, 10.0, 30.0, 100.0, 300.0, 1000.0],
        bound_fluid_pct: 8.5,
        free_fluid_pct: 14.2,
    };
    let bytes = encode_to_vec(&val).expect("encode NMR log");
    let (decoded, _): (FormationLog, usize) = decode_from_slice(&bytes).expect("decode NMR log");
    assert_eq!(val, decoded);
}

// --- Test 10: Production separator operating state ---
#[test]
fn test_separator_operating_roundtrip() {
    let val = SeparatorState::Operating {
        oil_rate_bpd: 3200.0,
        gas_rate_mscfd: 4800.0,
        water_rate_bpd: 1100.0,
        pressure_psi: 285.0,
        temperature_f: 145.0,
    };
    let bytes = encode_to_vec(&val).expect("encode separator operating");
    let (decoded, _): (SeparatorState, usize) =
        decode_from_slice(&bytes).expect("decode separator operating");
    assert_eq!(val, decoded);
}

// --- Test 11: Separator emergency shutdown ---
#[test]
fn test_separator_emergency_roundtrip() {
    let val = SeparatorState::Emergency("High-high level alarm on V-101".to_string());
    let bytes = encode_to_vec(&val).expect("encode separator emergency");
    let (decoded, _): (SeparatorState, usize) =
        decode_from_slice(&bytes).expect("decode separator emergency");
    assert_eq!(val, decoded);
}

// --- Test 12: Smart MFL pigging run with anomalies ---
#[test]
fn test_pigging_run_with_anomalies_roundtrip() {
    let val = PiggingRun {
        run_id: 4421,
        pipeline_name: "Deepwater Export 16-inch".to_string(),
        pig: PigType::SmartMfl {
            sensor_count: 256,
            resolution_mm: 5.0,
        },
        launch_pressure_psi: 1200,
        receive_pressure_psi: 850,
        distance_miles: 78.4,
        anomalies_found: vec![
            PipelineAnomaly::MetalLoss {
                clock_position: 6,
                depth_pct: 32.5,
                length_inches: 4.2,
            },
            PipelineAnomaly::Dent {
                depth_pct: 2.1,
                associated_metal_loss: true,
            },
            PipelineAnomaly::Crack {
                orientation: CrackOrientation::Axial,
                length_inches: 1.8,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode pigging run");
    let (decoded, _): (PiggingRun, usize) = decode_from_slice(&bytes).expect("decode pigging run");
    assert_eq!(val, decoded);
}

// --- Test 13: Flare emergency mode ---
#[test]
fn test_flare_emergency_mode_roundtrip() {
    let val = FlareMode::Emergency {
        flow_mscfd: 12000.0,
        source_psv_ids: vec![
            "PSV-101A".to_string(),
            "PSV-101B".to_string(),
            "PSV-205".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode flare emergency");
    let (decoded, _): (FlareMode, usize) =
        decode_from_slice(&bytes).expect("decode flare emergency");
    assert_eq!(val, decoded);
}

// --- Test 14: Flare high rate with smoke suppression ---
#[test]
fn test_flare_high_rate_roundtrip() {
    let val = FlareMode::HighRate {
        flow_mscfd: 5500.0,
        smoke_suppression_active: true,
    };
    let bytes = encode_to_vec(&val).expect("encode flare high rate");
    let (decoded, _): (FlareMode, usize) =
        decode_from_slice(&bytes).expect("decode flare high rate");
    assert_eq!(val, decoded);
}

// --- Test 15: HSE environmental release ---
#[test]
fn test_hse_environmental_release_roundtrip() {
    let val = HseReport {
        report_id: 88210,
        location: "Platform Alpha - Cellar Deck".to_string(),
        timestamp_epoch: 1_709_500_000,
        incident: HseIncidentClass::EnvironmentalRelease {
            substance: "Produced water".to_string(),
            volume_bbl: 0.25,
            contained: true,
        },
        corrective_actions: vec![
            "Replace corroded drain valve".to_string(),
            "Install secondary containment bund".to_string(),
            "Revise inspection schedule".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode HSE env release");
    let (decoded, _): (HseReport, usize) =
        decode_from_slice(&bytes).expect("decode HSE env release");
    assert_eq!(val, decoded);
}

// --- Test 16: HSE process safety event ---
#[test]
fn test_hse_process_safety_event_roundtrip() {
    let val = HseIncidentClass::ProcessSafetyEvent {
        tier: 2,
        release_type: "Hydrocarbon gas".to_string(),
        overpressure_psi: Some(15.0),
    };
    let bytes = encode_to_vec(&val).expect("encode PSE");
    let (decoded, _): (HseIncidentClass, usize) = decode_from_slice(&bytes).expect("decode PSE");
    assert_eq!(val, decoded);
}

// --- Test 17: Coiled tubing well intervention ---
#[test]
fn test_coiled_tubing_intervention_roundtrip() {
    let val = WellIntervention::CoiledTubing {
        od_inches: 2.0,
        acid_volume_gal: 5000,
    };
    let bytes = encode_to_vec(&val).expect("encode CT intervention");
    let (decoded, _): (WellIntervention, usize) =
        decode_from_slice(&bytes).expect("decode CT intervention");
    assert_eq!(val, decoded);
}

// --- Test 18: Plug and abandon with cement plugs ---
#[test]
fn test_plug_and_abandon_roundtrip() {
    let val = WellIntervention::PlugAndAbandon {
        cement_plugs: vec![
            CementPlug {
                top_depth_ft: 8000,
                bottom_depth_ft: 8200,
                cement_class: "Class H".to_string(),
            },
            CementPlug {
                top_depth_ft: 4000,
                bottom_depth_ft: 4150,
                cement_class: "Class G".to_string(),
            },
            CementPlug {
                top_depth_ft: 50,
                bottom_depth_ft: 200,
                cement_class: "Class A".to_string(),
            },
        ],
        surface_restoration: true,
    };
    let bytes = encode_to_vec(&val).expect("encode P&A");
    let (decoded, _): (WellIntervention, usize) = decode_from_slice(&bytes).expect("decode P&A");
    assert_eq!(val, decoded);
}

// --- Test 19: Three-phase reservoir cell ---
#[test]
fn test_reservoir_cell_three_phase_roundtrip() {
    let val = ReservoirCell {
        i: 42,
        j: 87,
        k: 3,
        porosity: 0.22,
        permeability_md: 150.0,
        pressure_psi: 4350.0,
        temperature_f: 210.0,
        phase: ReservoirCellPhase::ThreePhase {
            oil_saturation: 0.55,
            water_saturation: 0.30,
            gas_saturation: 0.15,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode reservoir cell");
    let (decoded, _): (ReservoirCell, usize) =
        decode_from_slice(&bytes).expect("decode reservoir cell");
    assert_eq!(val, decoded);
}

// --- Test 20: ESP artificial lift ---
#[test]
fn test_esp_artificial_lift_roundtrip() {
    let val = ArtificialLiftMethod::Esp {
        pump_stages: 120,
        frequency_hz: 55.0,
        intake_pressure_psi: 1800.0,
        motor_amps: 48.5,
        motor_temp_f: 275.0,
    };
    let bytes = encode_to_vec(&val).expect("encode ESP lift");
    let (decoded, _): (ArtificialLiftMethod, usize) =
        decode_from_slice(&bytes).expect("decode ESP lift");
    assert_eq!(val, decoded);
}

// --- Test 21: Gas lift with valve depths ---
#[test]
fn test_gas_lift_roundtrip() {
    let val = ArtificialLiftMethod::GasLift {
        injection_rate_mscfd: 800.0,
        injection_pressure_psi: 1400.0,
        valve_depths_ft: vec![2000, 4000, 6000, 8000, 9500],
        operating_valve_index: 4,
    };
    let bytes = encode_to_vec(&val).expect("encode gas lift");
    let (decoded, _): (ArtificialLiftMethod, usize) =
        decode_from_slice(&bytes).expect("decode gas lift");
    assert_eq!(val, decoded);
}

// --- Test 22: Drill string with mixed components ---
#[test]
fn test_drill_string_components_roundtrip() {
    let val: Vec<DrillStringComponent> = vec![
        DrillStringComponent::DrillPipe {
            od_inches: 5.0,
            weight_ppf: 19.5,
            grade: "S-135".to_string(),
        },
        DrillStringComponent::HeavyWeight {
            od_inches: 5.0,
            length_ft: 90.0,
        },
        DrillStringComponent::DrillCollar {
            od_inches: 8.0,
            id_inches: 2.8125,
            length_ft: 30.0,
        },
        DrillStringComponent::Stabilizer {
            blade_od_inches: 12.125,
            blade_count: 3,
        },
        DrillStringComponent::Motor {
            bend_angle_deg: 1.5,
            flow_range_gpm: (400, 600),
        },
        DrillStringComponent::Mwd {
            gamma_enabled: true,
            survey_interval_ft: 90,
        },
        DrillStringComponent::Lwd {
            tools: vec![
                "Resistivity".to_string(),
                "Density".to_string(),
                "Neutron".to_string(),
            ],
        },
        DrillStringComponent::RssTool {
            tool_face_mode: "Push-the-bit".to_string(),
            build_rate_deg_per_100ft: 8.0,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode drill string");
    let (decoded, _): (Vec<DrillStringComponent>, usize) =
        decode_from_slice(&bytes).expect("decode drill string");
    assert_eq!(val, decoded);
}
