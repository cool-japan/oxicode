//! Advanced Zstd compression tests for OxiCode — Semiconductor Fabrication domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world semiconductor fab data: wafer lot tracking, photolithography
//! exposure parameters, etch process recipes, CMP planarization settings, ion
//! implantation dose/energy, thin film deposition rates, defect inspection results,
//! yield analysis per die, cleanroom environmental monitoring, EUV scanner metrics,
//! wafer probe test results, and die sort bin maps.

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
enum WaferSize {
    Mm200,
    Mm300,
    Mm450,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LotPriority {
    Normal,
    Hot,
    SuperHot,
    Engineering,
    Qualification,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ResistType {
    PositiveDuv,
    NegativeDuv,
    EuvChemicallyAmplified,
    ElectronBeam,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EtchChemistry {
    Cl2Bcl3,
    Cf4O2,
    Sf6,
    Chf3Ar,
    HbrO2,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DepositionMethod {
    Pecvd,
    Lpcvd,
    Ald,
    Pvd,
    EBeamEvaporation,
    Mocvd,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DefectClass {
    Particle,
    Scratch,
    Residue,
    Pattern,
    CrystalOriginatedPit,
    Void,
    Bridging,
    Missing,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BinCategory {
    Pass,
    FailSpeed,
    FailLeakage,
    FailFunctional,
    FailIddq,
    Ink,
    Edge,
    Untested,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ImplantSpecies {
    Boron,
    Phosphorus,
    Arsenic,
    BF2,
    Indium,
    Antimony,
    Germanium,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SlurryType {
    SilicaAbrasive,
    CeriaAbrasive,
    AluminaAbrasive,
    DiamondSlurry,
}

/// Wafer lot tracking information.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferLot {
    lot_id: String,
    product_name: String,
    wafer_size: WaferSize,
    wafer_count: u8,
    priority: LotPriority,
    current_step: u32,
    total_steps: u32,
    start_timestamp_ms: u64,
    wafer_ids: Vec<String>,
}

/// Photolithography exposure parameters for a single layer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LithoExposure {
    layer_name: String,
    resist_type: ResistType,
    exposure_dose_mj_cm2: u32,
    focus_offset_nm: i32,
    numerical_aperture_x1000: u16,
    sigma_inner_x1000: u16,
    sigma_outer_x1000: u16,
    overlay_spec_x_nm: i32,
    overlay_spec_y_nm: i32,
    alignment_marks: Vec<(i32, i32)>,
}

/// Etch process recipe for a single step.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtchRecipe {
    recipe_id: String,
    chemistry: EtchChemistry,
    pressure_mtorr: u32,
    rf_power_source_w: u32,
    rf_power_bias_w: u32,
    temperature_mc: u32,
    gas_flow_sccm: Vec<(String, u32)>,
    etch_time_ms: u64,
    target_depth_nm: u32,
    selectivity_x100: u32,
}

/// CMP (Chemical Mechanical Planarization) settings.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmpSettings {
    step_name: String,
    slurry: SlurryType,
    down_force_psi_x100: u32,
    platen_rpm: u16,
    carrier_rpm: u16,
    slurry_flow_ml_min: u16,
    polish_time_sec: u32,
    removal_rate_nm_min: u32,
    within_wafer_non_uniformity_pct_x100: u16,
    endpoint_signal: Vec<u32>,
}

/// Ion implantation parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IonImplant {
    implant_id: String,
    species: ImplantSpecies,
    energy_kev_x10: u32,
    dose_atoms_cm2_exp: u8,
    dose_atoms_cm2_mantissa_x1000: u32,
    tilt_deg_x100: u16,
    twist_deg_x100: u16,
    beam_current_ua: u32,
    wafer_temperature_mc: u32,
}

/// Thin film deposition run.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThinFilmDeposition {
    run_id: u64,
    method: DepositionMethod,
    target_thickness_angstrom: u32,
    measured_thickness_angstrom: u32,
    deposition_rate_angstrom_min: u32,
    substrate_temp_mc: u32,
    chamber_pressure_mtorr: u32,
    precursors: Vec<String>,
    uniformity_pct_x100: u16,
    refractive_index_x10000: u32,
}

/// Defect inspection result for a single wafer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectInspection {
    wafer_id: String,
    inspection_step: String,
    total_defects: u32,
    defect_density_per_cm2_x100: u32,
    defect_entries: Vec<DefectEntry>,
}

/// A single defect on a wafer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectEntry {
    x_um: i32,
    y_um: i32,
    size_nm: u32,
    class: DefectClass,
    die_x: i16,
    die_y: i16,
}

/// Yield analysis for a wafer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldAnalysis {
    lot_id: String,
    wafer_id: String,
    total_die: u32,
    good_die: u32,
    yield_pct_x100: u16,
    die_results: Vec<u8>,
    edge_exclusion_mm: u8,
    bin_summary: Vec<(BinCategory, u32)>,
}

/// Cleanroom environmental monitoring snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CleanroomMonitor {
    sensor_id: u32,
    timestamp_ms: u64,
    temperature_mc: u32,
    humidity_pct_x100: u16,
    particle_count_0_1um: u32,
    particle_count_0_3um: u32,
    particle_count_0_5um: u32,
    pressure_diff_pa_x100: i32,
    iso_class: u8,
    alarm_active: bool,
}

/// EUV scanner operational metrics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EuvScannerMetrics {
    scanner_id: String,
    source_power_w: u32,
    collector_reflectivity_pct_x100: u16,
    dose_stability_pct_x10000: u16,
    overlay_x_nm_x10: i32,
    overlay_y_nm_x10: i32,
    throughput_wph: u16,
    pellicle_transmission_pct_x100: u16,
    reticle_heating_mc: u32,
    focus_uniformity_nm_x10: Vec<i16>,
}

/// Wafer probe test results.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferProbeResult {
    wafer_id: String,
    prober_id: String,
    test_program: String,
    tested_die: u32,
    pass_die: u32,
    total_test_time_ms: u64,
    parametric_measurements: Vec<ParametricMeasurement>,
}

/// A single parametric measurement from probe test.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParametricMeasurement {
    test_name: String,
    die_x: i16,
    die_y: i16,
    value_x1000: i64,
    lower_limit_x1000: i64,
    upper_limit_x1000: i64,
    pass: bool,
}

/// Die sort bin map for a wafer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieSortBinMap {
    wafer_id: String,
    product: String,
    die_size_x_um: u32,
    die_size_y_um: u32,
    map_cols: u16,
    map_rows: u16,
    bins: Vec<u8>,
    bin_definitions: Vec<(u8, BinCategory, String)>,
}

/// Diffusion furnace process record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiffusionFurnace {
    furnace_id: String,
    recipe_name: String,
    boat_slot_count: u8,
    zone_temps_mc: Vec<u32>,
    gas_flows_sccm: Vec<(String, u32)>,
    ramp_rate_mc_per_min: u32,
    soak_time_sec: u32,
    oxide_thickness_angstrom: u32,
}

/// Metrology measurement from an ellipsometer or CD-SEM.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetrologyMeasurement {
    wafer_id: String,
    tool_id: String,
    measurement_type: String,
    site_measurements: Vec<SiteMeasurement>,
    mean_x1000: i64,
    stddev_x1000: u64,
    spec_low_x1000: i64,
    spec_high_x1000: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiteMeasurement {
    site_id: u16,
    x_mm_x100: i32,
    y_mm_x100: i32,
    value_x1000: i64,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_wafer_lot(lot_num: u32) -> WaferLot {
    WaferLot {
        lot_id: format!("LOT-{lot_num:06}"),
        product_name: format!("ASIC-7nm-{}", lot_num % 10),
        wafer_size: WaferSize::Mm300,
        wafer_count: 25,
        priority: LotPriority::Hot,
        current_step: 42,
        total_steps: 380,
        start_timestamp_ms: 1_700_000_000_000 + (lot_num as u64) * 3_600_000,
        wafer_ids: (1u8..=25)
            .map(|w| format!("LOT-{lot_num:06}-W{w:02}"))
            .collect(),
    }
}

fn make_litho_exposure(layer: &str) -> LithoExposure {
    LithoExposure {
        layer_name: layer.to_string(),
        resist_type: ResistType::EuvChemicallyAmplified,
        exposure_dose_mj_cm2: 33_000,
        focus_offset_nm: -15,
        numerical_aperture_x1000: 330,
        sigma_inner_x1000: 200,
        sigma_outer_x1000: 800,
        overlay_spec_x_nm: 2,
        overlay_spec_y_nm: -1,
        alignment_marks: (-3i32..=3)
            .flat_map(|x| (-3i32..=3).map(move |y| (x * 30_000, y * 30_000)))
            .collect(),
    }
}

fn make_defect_entries(count: u32) -> Vec<DefectEntry> {
    (0..count)
        .map(|i| {
            let angle = (i as i32) * 137;
            DefectEntry {
                x_um: (angle * 311) % 150_000 - 75_000,
                y_um: (angle * 479) % 150_000 - 75_000,
                size_nm: 30 + (i * 7) % 500,
                class: match i % 8 {
                    0 => DefectClass::Particle,
                    1 => DefectClass::Scratch,
                    2 => DefectClass::Residue,
                    3 => DefectClass::Pattern,
                    4 => DefectClass::CrystalOriginatedPit,
                    5 => DefectClass::Void,
                    6 => DefectClass::Bridging,
                    _ => DefectClass::Missing,
                },
                die_x: ((i as i16) % 30) - 15,
                die_y: ((i as i16) % 26) - 13,
            }
        })
        .collect()
}

fn make_parametric_measurements(count: u32) -> Vec<ParametricMeasurement> {
    (0..count)
        .map(|i| ParametricMeasurement {
            test_name: format!("VTH_NMOS_{i}"),
            die_x: ((i % 30) as i16) - 15,
            die_y: ((i % 26) as i16) - 13,
            value_x1000: 350_000 + (i as i64 * 37) % 20_000,
            lower_limit_x1000: 300_000,
            upper_limit_x1000: 450_000,
            pass: true,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// 1. Round-trip for wafer lot tracking.
#[test]
fn test_zstd_wafer_lot_roundtrip() {
    let lot = make_wafer_lot(100);
    let encoded = encode_to_vec(&lot).expect("encode WaferLot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress WaferLot failed");
    let decompressed = decompress(&compressed).expect("decompress WaferLot failed");
    let (decoded, _): (WaferLot, usize) =
        decode_from_slice(&decompressed).expect("decode WaferLot failed");
    assert_eq!(lot, decoded);
}

/// 2. Round-trip for photolithography exposure parameters.
#[test]
fn test_zstd_litho_exposure_roundtrip() {
    let exposure = make_litho_exposure("M1");
    let encoded = encode_to_vec(&exposure).expect("encode LithoExposure failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress LithoExposure failed");
    let decompressed = decompress(&compressed).expect("decompress LithoExposure failed");
    let (decoded, _): (LithoExposure, usize) =
        decode_from_slice(&decompressed).expect("decode LithoExposure failed");
    assert_eq!(exposure, decoded);
}

/// 3. Round-trip for etch process recipe.
#[test]
fn test_zstd_etch_recipe_roundtrip() {
    let recipe = EtchRecipe {
        recipe_id: "ETCH-GATE-001".to_string(),
        chemistry: EtchChemistry::HbrO2,
        pressure_mtorr: 5_000,
        rf_power_source_w: 800,
        rf_power_bias_w: 150,
        temperature_mc: 60_000,
        gas_flow_sccm: vec![
            ("HBr".to_string(), 200),
            ("O2".to_string(), 5),
            ("He".to_string(), 400),
        ],
        etch_time_ms: 45_000,
        target_depth_nm: 55,
        selectivity_x100: 2500,
    };
    let encoded = encode_to_vec(&recipe).expect("encode EtchRecipe failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress EtchRecipe failed");
    let decompressed = decompress(&compressed).expect("decompress EtchRecipe failed");
    let (decoded, _): (EtchRecipe, usize) =
        decode_from_slice(&decompressed).expect("decode EtchRecipe failed");
    assert_eq!(recipe, decoded);
}

/// 4. Round-trip for CMP settings with endpoint signal trace.
#[test]
fn test_zstd_cmp_settings_roundtrip() {
    let cmp = CmpSettings {
        step_name: "STI-CMP-STEP1".to_string(),
        slurry: SlurryType::CeriaAbrasive,
        down_force_psi_x100: 300,
        platen_rpm: 93,
        carrier_rpm: 87,
        slurry_flow_ml_min: 200,
        polish_time_sec: 120,
        removal_rate_nm_min: 150,
        within_wafer_non_uniformity_pct_x100: 350,
        endpoint_signal: (0u32..600)
            .map(|t| 10_000 + t * 5 + (t * t) % 200)
            .collect(),
    };
    let encoded = encode_to_vec(&cmp).expect("encode CmpSettings failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress CmpSettings failed");
    let decompressed = decompress(&compressed).expect("decompress CmpSettings failed");
    let (decoded, _): (CmpSettings, usize) =
        decode_from_slice(&decompressed).expect("decode CmpSettings failed");
    assert_eq!(cmp, decoded);
}

/// 5. Round-trip for ion implantation parameters.
#[test]
fn test_zstd_ion_implant_roundtrip() {
    let implant = IonImplant {
        implant_id: "IMP-HALO-NMOS".to_string(),
        species: ImplantSpecies::Boron,
        energy_kev_x10: 100,
        dose_atoms_cm2_exp: 13,
        dose_atoms_cm2_mantissa_x1000: 3_500,
        tilt_deg_x100: 2800,
        twist_deg_x100: 0,
        beam_current_ua: 500,
        wafer_temperature_mc: 25_000,
    };
    let encoded = encode_to_vec(&implant).expect("encode IonImplant failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress IonImplant failed");
    let decompressed = decompress(&compressed).expect("decompress IonImplant failed");
    let (decoded, _): (IonImplant, usize) =
        decode_from_slice(&decompressed).expect("decode IonImplant failed");
    assert_eq!(implant, decoded);
}

/// 6. Round-trip for thin film deposition run.
#[test]
fn test_zstd_thin_film_deposition_roundtrip() {
    let dep = ThinFilmDeposition {
        run_id: 88_001,
        method: DepositionMethod::Ald,
        target_thickness_angstrom: 200,
        measured_thickness_angstrom: 198,
        deposition_rate_angstrom_min: 2,
        substrate_temp_mc: 300_000,
        chamber_pressure_mtorr: 500,
        precursors: vec!["TMA".to_string(), "H2O".to_string()],
        uniformity_pct_x100: 150,
        refractive_index_x10000: 17_600,
    };
    let encoded = encode_to_vec(&dep).expect("encode ThinFilmDeposition failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress ThinFilmDeposition failed");
    let decompressed = decompress(&compressed).expect("decompress ThinFilmDeposition failed");
    let (decoded, _): (ThinFilmDeposition, usize) =
        decode_from_slice(&decompressed).expect("decode ThinFilmDeposition failed");
    assert_eq!(dep, decoded);
}

/// 7. Round-trip for defect inspection with many defect entries.
#[test]
fn test_zstd_defect_inspection_roundtrip() {
    let entries = make_defect_entries(200);
    let inspection = DefectInspection {
        wafer_id: "LOT-100000-W01".to_string(),
        inspection_step: "POST-ETCH-M1".to_string(),
        total_defects: 200,
        defect_density_per_cm2_x100: 285,
        defect_entries: entries,
    };
    let encoded = encode_to_vec(&inspection).expect("encode DefectInspection failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress DefectInspection failed");
    let decompressed = decompress(&compressed).expect("decompress DefectInspection failed");
    let (decoded, _): (DefectInspection, usize) =
        decode_from_slice(&decompressed).expect("decode DefectInspection failed");
    assert_eq!(inspection, decoded);
}

/// 8. Round-trip for yield analysis with die-level results.
#[test]
fn test_zstd_yield_analysis_roundtrip() {
    let analysis = YieldAnalysis {
        lot_id: "LOT-200000".to_string(),
        wafer_id: "LOT-200000-W12".to_string(),
        total_die: 600,
        good_die: 558,
        yield_pct_x100: 9300,
        die_results: (0u32..600)
            .map(|i| if i % 15 == 0 { 2 } else { 1 })
            .collect::<Vec<u32>>()
            .into_iter()
            .map(|v| v as u8)
            .collect(),
        edge_exclusion_mm: 3,
        bin_summary: vec![
            (BinCategory::Pass, 558),
            (BinCategory::FailSpeed, 12),
            (BinCategory::FailLeakage, 8),
            (BinCategory::FailFunctional, 5),
            (BinCategory::Edge, 17),
        ],
    };
    let encoded = encode_to_vec(&analysis).expect("encode YieldAnalysis failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress YieldAnalysis failed");
    let decompressed = decompress(&compressed).expect("decompress YieldAnalysis failed");
    let (decoded, _): (YieldAnalysis, usize) =
        decode_from_slice(&decompressed).expect("decode YieldAnalysis failed");
    assert_eq!(analysis, decoded);
}

/// 9. Round-trip for cleanroom environmental monitoring.
#[test]
fn test_zstd_cleanroom_monitor_roundtrip() {
    let monitor = CleanroomMonitor {
        sensor_id: 42,
        timestamp_ms: 1_700_100_000_000,
        temperature_mc: 21_500,
        humidity_pct_x100: 4500,
        particle_count_0_1um: 3,
        particle_count_0_3um: 1,
        particle_count_0_5um: 0,
        pressure_diff_pa_x100: 1250,
        iso_class: 4,
        alarm_active: false,
    };
    let encoded = encode_to_vec(&monitor).expect("encode CleanroomMonitor failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress CleanroomMonitor failed");
    let decompressed = decompress(&compressed).expect("decompress CleanroomMonitor failed");
    let (decoded, _): (CleanroomMonitor, usize) =
        decode_from_slice(&decompressed).expect("decode CleanroomMonitor failed");
    assert_eq!(monitor, decoded);
}

/// 10. Round-trip for EUV scanner metrics with focus uniformity map.
#[test]
fn test_zstd_euv_scanner_metrics_roundtrip() {
    let metrics = EuvScannerMetrics {
        scanner_id: "NXE3600D-001".to_string(),
        source_power_w: 400,
        collector_reflectivity_pct_x100: 6700,
        dose_stability_pct_x10000: 9985,
        overlay_x_nm_x10: -12,
        overlay_y_nm_x10: 8,
        throughput_wph: 185,
        pellicle_transmission_pct_x100: 9000,
        reticle_heating_mc: 350,
        focus_uniformity_nm_x10: (-20i16..=20)
            .flat_map(|x| (-20i16..=20).map(move |y| (x * 3 + y * 2) % 50))
            .collect(),
    };
    let encoded = encode_to_vec(&metrics).expect("encode EuvScannerMetrics failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress EuvScannerMetrics failed");
    let decompressed = decompress(&compressed).expect("decompress EuvScannerMetrics failed");
    let (decoded, _): (EuvScannerMetrics, usize) =
        decode_from_slice(&decompressed).expect("decode EuvScannerMetrics failed");
    assert_eq!(metrics, decoded);
}

/// 11. Round-trip for wafer probe test results with parametric data.
#[test]
fn test_zstd_wafer_probe_result_roundtrip() {
    let result = WaferProbeResult {
        wafer_id: "LOT-300000-W05".to_string(),
        prober_id: "PROBER-A3".to_string(),
        test_program: "7NM_SOC_FINAL_V2".to_string(),
        tested_die: 580,
        pass_die: 540,
        total_test_time_ms: 3_600_000,
        parametric_measurements: make_parametric_measurements(100),
    };
    let encoded = encode_to_vec(&result).expect("encode WaferProbeResult failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress WaferProbeResult failed");
    let decompressed = decompress(&compressed).expect("decompress WaferProbeResult failed");
    let (decoded, _): (WaferProbeResult, usize) =
        decode_from_slice(&decompressed).expect("decode WaferProbeResult failed");
    assert_eq!(result, decoded);
}

/// 12. Round-trip for die sort bin map.
#[test]
fn test_zstd_die_sort_bin_map_roundtrip() {
    let bin_map = DieSortBinMap {
        wafer_id: "LOT-400000-W20".to_string(),
        product: "GPU-5NM-HPC".to_string(),
        die_size_x_um: 15_000,
        die_size_y_um: 12_000,
        map_cols: 20,
        map_rows: 24,
        bins: (0u32..480)
            .map(|i| match i % 20 {
                0 => 2,
                7 => 3,
                13 => 4,
                _ => 1,
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .map(|v| v as u8)
            .collect(),
        bin_definitions: vec![
            (1, BinCategory::Pass, "Good".to_string()),
            (2, BinCategory::FailSpeed, "Speed fail".to_string()),
            (3, BinCategory::FailLeakage, "IDDQ fail".to_string()),
            (4, BinCategory::FailFunctional, "Scan fail".to_string()),
        ],
    };
    let encoded = encode_to_vec(&bin_map).expect("encode DieSortBinMap failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress DieSortBinMap failed");
    let decompressed = decompress(&compressed).expect("decompress DieSortBinMap failed");
    let (decoded, _): (DieSortBinMap, usize) =
        decode_from_slice(&decompressed).expect("decode DieSortBinMap failed");
    assert_eq!(bin_map, decoded);
}

/// 13. Round-trip for diffusion furnace process record.
#[test]
fn test_zstd_diffusion_furnace_roundtrip() {
    let furnace = DiffusionFurnace {
        furnace_id: "FURN-OX-03".to_string(),
        recipe_name: "GATE-OX-12A".to_string(),
        boat_slot_count: 150,
        zone_temps_mc: vec![1_050_000, 1_050_500, 1_051_000, 1_050_800, 1_050_200],
        gas_flows_sccm: vec![
            ("O2".to_string(), 5_000),
            ("N2".to_string(), 10_000),
            ("HCl".to_string(), 50),
        ],
        ramp_rate_mc_per_min: 5_000,
        soak_time_sec: 1_800,
        oxide_thickness_angstrom: 12,
    };
    let encoded = encode_to_vec(&furnace).expect("encode DiffusionFurnace failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress DiffusionFurnace failed");
    let decompressed = decompress(&compressed).expect("decompress DiffusionFurnace failed");
    let (decoded, _): (DiffusionFurnace, usize) =
        decode_from_slice(&decompressed).expect("decode DiffusionFurnace failed");
    assert_eq!(furnace, decoded);
}

/// 14. Round-trip for metrology measurements with site data.
#[test]
fn test_zstd_metrology_measurement_roundtrip() {
    let sites: Vec<SiteMeasurement> = (0u16..49)
        .map(|i| {
            let row = (i / 7) as i32 - 3;
            let col = (i % 7) as i32 - 3;
            SiteMeasurement {
                site_id: i,
                x_mm_x100: col * 2_000,
                y_mm_x100: row * 2_000,
                value_x1000: 200_000 + (row * col * 100) as i64,
            }
        })
        .collect();
    let metrology = MetrologyMeasurement {
        wafer_id: "LOT-500000-W01".to_string(),
        tool_id: "ELLIP-KLA-01".to_string(),
        measurement_type: "FILM_THICKNESS".to_string(),
        site_measurements: sites,
        mean_x1000: 200_150,
        stddev_x1000: 1_200,
        spec_low_x1000: 195_000,
        spec_high_x1000: 205_000,
    };
    let encoded = encode_to_vec(&metrology).expect("encode MetrologyMeasurement failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress MetrologyMeasurement failed");
    let decompressed = decompress(&compressed).expect("decompress MetrologyMeasurement failed");
    let (decoded, _): (MetrologyMeasurement, usize) =
        decode_from_slice(&decompressed).expect("decode MetrologyMeasurement failed");
    assert_eq!(metrology, decoded);
}

/// 15. Compression ratio check — repetitive bin map data should compress well.
#[test]
fn test_zstd_bin_map_compression_ratio() {
    let bins: Vec<u8> = (0u32..10_000)
        .map(|i| if i % 50 == 0 { 2 } else { 1 })
        .collect::<Vec<u32>>()
        .into_iter()
        .map(|v| v as u8)
        .collect();
    let encoded = encode_to_vec(&bins).expect("encode bin map data failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress bin map data failed");
    assert!(
        compressed.len() < encoded.len(),
        "expected compressed size {} < encoded size {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("decompress bin map data failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode bin map data failed");
    assert_eq!(bins, decoded);
}

/// 16. Round-trip for a batch of wafer lots.
#[test]
fn test_zstd_batch_wafer_lots_roundtrip() {
    let lots: Vec<WaferLot> = (0u32..10).map(|i| make_wafer_lot(i * 1000)).collect();
    let encoded = encode_to_vec(&lots).expect("encode Vec<WaferLot> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Vec<WaferLot> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<WaferLot> failed");
    let (decoded, _): (Vec<WaferLot>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<WaferLot> failed");
    assert_eq!(lots, decoded);
}

/// 17. Round-trip for multiple litho layers.
#[test]
fn test_zstd_multi_layer_litho_roundtrip() {
    let layers: Vec<LithoExposure> = ["ACTIVE", "POLY", "M1", "M2", "M3", "M4", "VIA1", "VIA2"]
        .iter()
        .map(|name| make_litho_exposure(name))
        .collect();
    let encoded = encode_to_vec(&layers).expect("encode Vec<LithoExposure> failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Vec<LithoExposure> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<LithoExposure> failed");
    let (decoded, _): (Vec<LithoExposure>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<LithoExposure> failed");
    assert_eq!(layers, decoded);
}

/// 18. Round-trip for multiple ion implant steps in a process flow.
#[test]
fn test_zstd_implant_process_flow_roundtrip() {
    let implants = vec![
        IonImplant {
            implant_id: "WELL-NWELL".to_string(),
            species: ImplantSpecies::Phosphorus,
            energy_kev_x10: 3_400,
            dose_atoms_cm2_exp: 13,
            dose_atoms_cm2_mantissa_x1000: 1_500,
            tilt_deg_x100: 700,
            twist_deg_x100: 0,
            beam_current_ua: 2_000,
            wafer_temperature_mc: 25_000,
        },
        IonImplant {
            implant_id: "WELL-PWELL".to_string(),
            species: ImplantSpecies::Boron,
            energy_kev_x10: 1_800,
            dose_atoms_cm2_exp: 13,
            dose_atoms_cm2_mantissa_x1000: 2_000,
            tilt_deg_x100: 700,
            twist_deg_x100: 18_000,
            beam_current_ua: 1_500,
            wafer_temperature_mc: 25_000,
        },
        IonImplant {
            implant_id: "VT-ADJUST-NMOS".to_string(),
            species: ImplantSpecies::Indium,
            energy_kev_x10: 800,
            dose_atoms_cm2_exp: 12,
            dose_atoms_cm2_mantissa_x1000: 5_000,
            tilt_deg_x100: 0,
            twist_deg_x100: 0,
            beam_current_ua: 300,
            wafer_temperature_mc: 25_000,
        },
        IonImplant {
            implant_id: "S/D-NMOS".to_string(),
            species: ImplantSpecies::Arsenic,
            energy_kev_x10: 300,
            dose_atoms_cm2_exp: 15,
            dose_atoms_cm2_mantissa_x1000: 3_000,
            tilt_deg_x100: 0,
            twist_deg_x100: 0,
            beam_current_ua: 5_000,
            wafer_temperature_mc: 25_000,
        },
    ];
    let encoded = encode_to_vec(&implants).expect("encode implant flow failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress implant flow failed");
    let decompressed = decompress(&compressed).expect("decompress implant flow failed");
    let (decoded, _): (Vec<IonImplant>, usize) =
        decode_from_slice(&decompressed).expect("decode implant flow failed");
    assert_eq!(implants, decoded);
}

/// 19. Round-trip for time-series cleanroom sensor readings.
#[test]
fn test_zstd_cleanroom_time_series_roundtrip() {
    let readings: Vec<CleanroomMonitor> = (0u64..200)
        .map(|t| CleanroomMonitor {
            sensor_id: 7,
            timestamp_ms: 1_700_100_000_000 + t * 60_000,
            temperature_mc: 21_500 + (t as u32 % 50),
            humidity_pct_x100: 4500 + (t as u16 % 100),
            particle_count_0_1um: t as u32 % 5,
            particle_count_0_3um: t as u32 % 3,
            particle_count_0_5um: t as u32 % 2,
            pressure_diff_pa_x100: 1250 + (t as i32 % 30) - 15,
            iso_class: 4,
            alarm_active: t % 200 == 199,
        })
        .collect();
    let encoded = encode_to_vec(&readings).expect("encode cleanroom series failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress cleanroom series failed");
    let decompressed = decompress(&compressed).expect("decompress cleanroom series failed");
    let (decoded, _): (Vec<CleanroomMonitor>, usize) =
        decode_from_slice(&decompressed).expect("decode cleanroom series failed");
    assert_eq!(readings, decoded);
}

/// 20. Round-trip for a complete fab etch recipe sequence.
#[test]
fn test_zstd_etch_recipe_sequence_roundtrip() {
    let recipes = vec![
        EtchRecipe {
            recipe_id: "HARD-MASK-OPEN".to_string(),
            chemistry: EtchChemistry::Cf4O2,
            pressure_mtorr: 10_000,
            rf_power_source_w: 500,
            rf_power_bias_w: 100,
            temperature_mc: 40_000,
            gas_flow_sccm: vec![("CF4".to_string(), 100), ("O2".to_string(), 10)],
            etch_time_ms: 30_000,
            target_depth_nm: 80,
            selectivity_x100: 5000,
        },
        EtchRecipe {
            recipe_id: "MAIN-ETCH".to_string(),
            chemistry: EtchChemistry::Cl2Bcl3,
            pressure_mtorr: 4_000,
            rf_power_source_w: 700,
            rf_power_bias_w: 200,
            temperature_mc: 55_000,
            gas_flow_sccm: vec![
                ("Cl2".to_string(), 150),
                ("BCl3".to_string(), 50),
                ("N2".to_string(), 20),
            ],
            etch_time_ms: 60_000,
            target_depth_nm: 120,
            selectivity_x100: 3000,
        },
        EtchRecipe {
            recipe_id: "OVER-ETCH".to_string(),
            chemistry: EtchChemistry::HbrO2,
            pressure_mtorr: 8_000,
            rf_power_source_w: 300,
            rf_power_bias_w: 50,
            temperature_mc: 50_000,
            gas_flow_sccm: vec![("HBr".to_string(), 250), ("O2".to_string(), 3)],
            etch_time_ms: 15_000,
            target_depth_nm: 10,
            selectivity_x100: 10000,
        },
    ];
    let encoded = encode_to_vec(&recipes).expect("encode etch sequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress etch sequence failed");
    let decompressed = decompress(&compressed).expect("decompress etch sequence failed");
    let (decoded, _): (Vec<EtchRecipe>, usize) =
        decode_from_slice(&decompressed).expect("decode etch sequence failed");
    assert_eq!(recipes, decoded);
}

/// 21. Round-trip for multiple thin film depositions in a BEOL stack.
#[test]
fn test_zstd_beol_film_stack_roundtrip() {
    let films = vec![
        ThinFilmDeposition {
            run_id: 90_001,
            method: DepositionMethod::Pecvd,
            target_thickness_angstrom: 3_000,
            measured_thickness_angstrom: 3_015,
            deposition_rate_angstrom_min: 1_200,
            substrate_temp_mc: 400_000,
            chamber_pressure_mtorr: 2_500,
            precursors: vec!["SiH4".to_string(), "N2O".to_string()],
            uniformity_pct_x100: 200,
            refractive_index_x10000: 14_600,
        },
        ThinFilmDeposition {
            run_id: 90_002,
            method: DepositionMethod::Pvd,
            target_thickness_angstrom: 250,
            measured_thickness_angstrom: 248,
            deposition_rate_angstrom_min: 500,
            substrate_temp_mc: 25_000,
            chamber_pressure_mtorr: 3,
            precursors: vec!["Ta".to_string()],
            uniformity_pct_x100: 300,
            refractive_index_x10000: 0,
        },
        ThinFilmDeposition {
            run_id: 90_003,
            method: DepositionMethod::Pvd,
            target_thickness_angstrom: 2_000,
            measured_thickness_angstrom: 1_990,
            deposition_rate_angstrom_min: 4_000,
            substrate_temp_mc: 50_000,
            chamber_pressure_mtorr: 2,
            precursors: vec!["Cu".to_string()],
            uniformity_pct_x100: 250,
            refractive_index_x10000: 0,
        },
    ];
    let encoded = encode_to_vec(&films).expect("encode BEOL film stack failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress BEOL film stack failed");
    let decompressed = decompress(&compressed).expect("decompress BEOL film stack failed");
    let (decoded, _): (Vec<ThinFilmDeposition>, usize) =
        decode_from_slice(&decompressed).expect("decode BEOL film stack failed");
    assert_eq!(films, decoded);
}

/// 22. Large defect map compression effectiveness and correctness.
#[test]
fn test_zstd_large_defect_map_roundtrip() {
    let entries = make_defect_entries(2_000);
    let inspection = DefectInspection {
        wafer_id: "LOT-999999-W25".to_string(),
        inspection_step: "FINAL-INSP".to_string(),
        total_defects: 2_000,
        defect_density_per_cm2_x100: 2_850,
        defect_entries: entries,
    };
    let encoded = encode_to_vec(&inspection).expect("encode large DefectInspection failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress large DefectInspection failed");
    assert!(
        compressed.len() < encoded.len(),
        "expected compressed size {} < encoded size {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large DefectInspection failed");
    let (decoded, _): (DefectInspection, usize) =
        decode_from_slice(&decompressed).expect("decode large DefectInspection failed");
    assert_eq!(inspection, decoded);
}
