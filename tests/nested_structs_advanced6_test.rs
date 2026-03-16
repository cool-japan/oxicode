//! Advanced nested struct encoding tests for OxiCode (set 6)
//! Theme: Semiconductor fabrication and chip manufacturing

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

// ---------------------------------------------------------------------------
// Domain types: Wafer lot tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieCoordinate {
    row: u16,
    col: u16,
    site: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferIdentifier {
    wafer_number: u8,
    slot: u8,
    orientation_notch_deg: f32,
    die_grid: Vec<DieCoordinate>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LotInfo {
    lot_id: String,
    product_code: String,
    technology_node_nm: u16,
    wafer_count: u8,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferLot {
    lot: LotInfo,
    wafers: Vec<WaferIdentifier>,
    route_id: String,
    current_step: u32,
    total_steps: u32,
}

// ---------------------------------------------------------------------------
// Photolithography parameters
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OverlayError {
    x_offset_nm: f64,
    y_offset_nm: f64,
    rotation_urad: f64,
    magnification_ppm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExposureSettings {
    dose_mj_per_cm2: f64,
    focus_offset_nm: f64,
    numerical_aperture: f64,
    sigma_inner: f64,
    sigma_outer: f64,
    wavelength_nm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReticleInfo {
    reticle_id: String,
    layer_name: String,
    field_size_x_mm: f64,
    field_size_y_mm: f64,
    pellicle_attached: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LithoStep {
    step_name: String,
    tool_id: String,
    reticle: ReticleInfo,
    exposure: ExposureSettings,
    overlay: OverlayError,
    alignment_marks_used: u8,
}

// ---------------------------------------------------------------------------
// Etch process recipes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GasFlow {
    gas_name: String,
    flow_sccm: f64,
    mfc_channel: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RfSource {
    frequency_mhz: f64,
    power_watts: f64,
    pulsed: bool,
    duty_cycle_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtchStep {
    step_label: String,
    duration_sec: f64,
    pressure_mtorr: f64,
    temperature_c: f64,
    gases: Vec<GasFlow>,
    source_rf: RfSource,
    bias_rf: RfSource,
    endpoint_signal: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtchRecipe {
    recipe_id: String,
    chamber_id: String,
    steps: Vec<EtchStep>,
    total_time_sec: f64,
}

// ---------------------------------------------------------------------------
// CMP settings
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlurryConfig {
    slurry_type: String,
    flow_rate_ml_per_min: f64,
    ph_value: f64,
    abrasive_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PadConditioner {
    conditioner_type: String,
    sweep_speed_rpm: f64,
    down_force_lbs: f64,
    in_situ: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmpProfile {
    platen_speed_rpm: f64,
    carrier_speed_rpm: f64,
    down_force_psi: f64,
    polish_time_sec: f64,
    slurry: SlurryConfig,
    conditioner: PadConditioner,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmpProcess {
    process_id: String,
    layer_target: String,
    profiles: Vec<CmpProfile>,
    post_clean_recipe: String,
}

// ---------------------------------------------------------------------------
// Ion implantation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantSpecies {
    element: String,
    isotope_mass: u16,
    charge_state: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantDose {
    dose_atoms_per_cm2: f64,
    energy_kev: f64,
    tilt_deg: f64,
    twist_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantProfile {
    profile_name: String,
    species: ImplantSpecies,
    dose_params: ImplantDose,
    beam_current_ua: f64,
    scan_mode: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IonImplantRecipe {
    recipe_id: String,
    tool_name: String,
    profiles: Vec<ImplantProfile>,
    anneal_required: bool,
    anneal_temp_c: Option<f64>,
}

// ---------------------------------------------------------------------------
// Metrology measurements
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CdSemMeasurement {
    feature_name: String,
    target_cd_nm: f64,
    measured_cd_nm: f64,
    lwr_nm: f64,
    lwe_nm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OcdMeasurement {
    profile_name: String,
    cd_top_nm: f64,
    cd_bottom_nm: f64,
    sidewall_angle_deg: f64,
    height_nm: f64,
    goodness_of_fit: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FilmThickness {
    film_type: String,
    target_nm: f64,
    measured_nm: f64,
    uniformity_pct: f64,
    measurement_points: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetrologyReport {
    wafer_id: String,
    step_name: String,
    cd_sem: Vec<CdSemMeasurement>,
    ocd: Vec<OcdMeasurement>,
    film: Vec<FilmThickness>,
    timestamp_epoch: u64,
}

// ---------------------------------------------------------------------------
// Yield analysis
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BinCode {
    bin_number: u16,
    bin_name: String,
    pass_fail: bool,
    count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieBinResult {
    die: DieCoordinate,
    hard_bin: u16,
    soft_bin: u16,
    test_time_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferMap {
    wafer_id: String,
    die_results: Vec<DieBinResult>,
    total_die: u32,
    good_die: u32,
    yield_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldReport {
    lot_id: String,
    wafer_maps: Vec<WaferMap>,
    bin_definitions: Vec<BinCode>,
    lot_yield_pct: f64,
}

// ---------------------------------------------------------------------------
// Clean room environment
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParticleCount {
    size_threshold_um: f64,
    count_per_cubic_ft: u32,
    measurement_location: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentReading {
    temperature_c: f64,
    humidity_pct: f64,
    pressure_pa: f64,
    airflow_m_per_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CleanRoomZone {
    zone_name: String,
    iso_class: u8,
    environment: EnvironmentReading,
    particle_counts: Vec<ParticleCount>,
    personnel_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FabEnvironment {
    fab_name: String,
    zones: Vec<CleanRoomZone>,
    total_area_sqm: f64,
    monitoring_interval_sec: u32,
}

// ---------------------------------------------------------------------------
// Diffusion furnace profiles
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceRamp {
    segment_name: String,
    start_temp_c: f64,
    end_temp_c: f64,
    ramp_rate_c_per_min: f64,
    hold_time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceGas {
    gas_name: String,
    flow_slm: f64,
    start_time_min: f64,
    stop_time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceRecipe {
    recipe_name: String,
    tube_id: String,
    ramp_segments: Vec<FurnaceRamp>,
    gas_profile: Vec<FurnaceGas>,
    boat_rotation_rpm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiffusionRun {
    run_id: String,
    lot: LotInfo,
    recipe: FurnaceRecipe,
    oxide_target_nm: f64,
    oxide_measured_nm: Option<f64>,
}

// ---------------------------------------------------------------------------
// CVD/PVD deposition
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrecursorGas {
    chemical_name: String,
    flow_sccm: f64,
    bubbler_temp_c: Option<f64>,
    carrier_gas: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionConditions {
    temperature_c: f64,
    pressure_torr: f64,
    rf_power_watts: Option<f64>,
    dc_power_kw: Option<f64>,
    substrate_bias_v: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionLayer {
    layer_name: String,
    method: String,
    precursors: Vec<PrecursorGas>,
    conditions: DepositionConditions,
    target_thickness_nm: f64,
    deposition_rate_nm_per_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionProcess {
    process_id: String,
    tool_id: String,
    layers: Vec<DepositionLayer>,
    total_time_min: f64,
}

// ---------------------------------------------------------------------------
// Electrical test results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransistorParams {
    device_type: String,
    channel_length_nm: f64,
    channel_width_nm: f64,
    vt_mv: f64,
    idsat_ua_per_um: f64,
    ioff_pa_per_um: f64,
    subthreshold_swing_mv_per_dec: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LeakageCurrent {
    junction_name: String,
    leakage_na: f64,
    voltage_v: f64,
    temperature_c: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtestSite {
    site_id: u16,
    die: DieCoordinate,
    transistors: Vec<TransistorParams>,
    leakage: Vec<LeakageCurrent>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtestWafer {
    wafer_id: String,
    sites: Vec<EtestSite>,
    pass: bool,
    median_vt_mv: f64,
}

// ---------------------------------------------------------------------------
// Packaging
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WireBondSpec {
    pad_name: String,
    wire_material: String,
    wire_diameter_um: f64,
    loop_height_um: f64,
    ball_size_um: f64,
    bond_force_gf: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlipChipBump {
    bump_id: String,
    material: String,
    diameter_um: f64,
    pitch_um: f64,
    height_um: f64,
    underfill: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PackageSubstrate {
    substrate_type: String,
    layer_count: u8,
    thickness_mm: f64,
    material: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PackageDesign {
    package_type: String,
    body_size_mm_x: f64,
    body_size_mm_y: f64,
    substrate: PackageSubstrate,
    wire_bonds: Vec<WireBondSpec>,
    flip_chip_bumps: Vec<FlipChipBump>,
    lead_count: u16,
}

// ---------------------------------------------------------------------------
// ESD protection
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdTestResult {
    model: String,
    voltage_kv: f64,
    pin_name: String,
    passed: bool,
    leakage_after_na: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdProtection {
    clamp_type: String,
    trigger_voltage_v: f64,
    holding_voltage_v: f64,
    on_resistance_ohm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdSpec {
    device_name: String,
    protection_cells: Vec<EsdProtection>,
    test_results: Vec<EsdTestResult>,
    classification: String,
}

// ---------------------------------------------------------------------------
// Defect classification
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectLocation {
    die: DieCoordinate,
    x_um: f64,
    y_um: f64,
    layer: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectFeature {
    size_um: f64,
    aspect_ratio: f64,
    brightness: f64,
    polarity: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectEntry {
    defect_id: u64,
    location: DefectLocation,
    feature: DefectFeature,
    classification: String,
    killer: bool,
    review_completed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectInspectionReport {
    wafer_id: String,
    tool_id: String,
    recipe_name: String,
    defects: Vec<DefectEntry>,
    total_defects: u32,
    killer_defects: u32,
    defect_density_per_cm2: f64,
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_wafer_lot_tracking() {
    let lot = WaferLot {
        lot: LotInfo {
            lot_id: "N7-2026-0315-A".into(),
            product_code: "APEX7".into(),
            technology_node_nm: 7,
            wafer_count: 25,
            priority: 1,
        },
        wafers: vec![
            WaferIdentifier {
                wafer_number: 1,
                slot: 1,
                orientation_notch_deg: 0.0,
                die_grid: vec![
                    DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 1,
                        col: 0,
                        site: 1,
                    },
                ],
            },
            WaferIdentifier {
                wafer_number: 2,
                slot: 2,
                orientation_notch_deg: 0.0,
                die_grid: vec![
                    DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 2,
                    },
                ],
            },
        ],
        route_id: "ROUTE-N7-LOGIC-V3".into(),
        current_step: 42,
        total_steps: 320,
    };
    let encoded = encode_to_vec(&lot).expect("encode wafer lot");
    let (decoded, _): (WaferLot, _) = decode_from_slice(&encoded).expect("decode wafer lot");
    assert_eq!(lot, decoded);
}

#[test]
fn test_photolithography_full_step() {
    let litho = LithoStep {
        step_name: "M1_PHOTO".into(),
        tool_id: "ASML-NXT2000-01".into(),
        reticle: ReticleInfo {
            reticle_id: "RTL-M1-V2R3".into(),
            layer_name: "METAL1".into(),
            field_size_x_mm: 26.0,
            field_size_y_mm: 33.0,
            pellicle_attached: true,
        },
        exposure: ExposureSettings {
            dose_mj_per_cm2: 30.5,
            focus_offset_nm: -10.0,
            numerical_aperture: 0.33,
            sigma_inner: 0.6,
            sigma_outer: 0.9,
            wavelength_nm: 13.5,
        },
        overlay: OverlayError {
            x_offset_nm: 0.8,
            y_offset_nm: -0.3,
            rotation_urad: 0.12,
            magnification_ppm: 0.05,
        },
        alignment_marks_used: 8,
    };
    let encoded = encode_to_vec(&litho).expect("encode litho step");
    let (decoded, _): (LithoStep, _) = decode_from_slice(&encoded).expect("decode litho step");
    assert_eq!(litho, decoded);
}

#[test]
fn test_etch_recipe_multi_step() {
    let recipe = EtchRecipe {
        recipe_id: "ETCH-POLY-HK-V4".into(),
        chamber_id: "LAM-KIYO-C3".into(),
        steps: vec![
            EtchStep {
                step_label: "Breakthrough".into(),
                duration_sec: 5.0,
                pressure_mtorr: 4.0,
                temperature_c: 60.0,
                gases: vec![
                    GasFlow {
                        gas_name: "Cl2".into(),
                        flow_sccm: 100.0,
                        mfc_channel: 1,
                    },
                    GasFlow {
                        gas_name: "BCl3".into(),
                        flow_sccm: 50.0,
                        mfc_channel: 2,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 800.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 50.0,
                    pulsed: true,
                    duty_cycle_pct: 30.0,
                },
                endpoint_signal: None,
            },
            EtchStep {
                step_label: "Main Etch".into(),
                duration_sec: 45.0,
                pressure_mtorr: 8.0,
                temperature_c: 55.0,
                gases: vec![
                    GasFlow {
                        gas_name: "HBr".into(),
                        flow_sccm: 200.0,
                        mfc_channel: 3,
                    },
                    GasFlow {
                        gas_name: "O2".into(),
                        flow_sccm: 5.0,
                        mfc_channel: 4,
                    },
                    GasFlow {
                        gas_name: "He".into(),
                        flow_sccm: 300.0,
                        mfc_channel: 5,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 400.0,
                    pulsed: true,
                    duty_cycle_pct: 50.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 120.0,
                    pulsed: true,
                    duty_cycle_pct: 40.0,
                },
                endpoint_signal: Some("OES-Br-827nm".into()),
            },
            EtchStep {
                step_label: "Over Etch".into(),
                duration_sec: 15.0,
                pressure_mtorr: 12.0,
                temperature_c: 55.0,
                gases: vec![
                    GasFlow {
                        gas_name: "HBr".into(),
                        flow_sccm: 150.0,
                        mfc_channel: 3,
                    },
                    GasFlow {
                        gas_name: "O2".into(),
                        flow_sccm: 10.0,
                        mfc_channel: 4,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 200.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 30.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                endpoint_signal: None,
            },
        ],
        total_time_sec: 65.0,
    };
    let encoded = encode_to_vec(&recipe).expect("encode etch recipe");
    let (decoded, _): (EtchRecipe, _) = decode_from_slice(&encoded).expect("decode etch recipe");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_cmp_polish_process() {
    let cmp = CmpProcess {
        process_id: "CMP-CU-DUAL-V2".into(),
        layer_target: "COPPER_M2".into(),
        profiles: vec![
            CmpProfile {
                platen_speed_rpm: 93.0,
                carrier_speed_rpm: 87.0,
                down_force_psi: 2.5,
                polish_time_sec: 60.0,
                slurry: SlurryConfig {
                    slurry_type: "Barrier-CMP".into(),
                    flow_rate_ml_per_min: 200.0,
                    ph_value: 9.5,
                    abrasive_pct: 3.0,
                },
                conditioner: PadConditioner {
                    conditioner_type: "Diamond-Disk".into(),
                    sweep_speed_rpm: 20.0,
                    down_force_lbs: 5.0,
                    in_situ: true,
                },
            },
            CmpProfile {
                platen_speed_rpm: 60.0,
                carrier_speed_rpm: 55.0,
                down_force_psi: 1.2,
                polish_time_sec: 30.0,
                slurry: SlurryConfig {
                    slurry_type: "Buff-Clean".into(),
                    flow_rate_ml_per_min: 300.0,
                    ph_value: 7.0,
                    abrasive_pct: 0.5,
                },
                conditioner: PadConditioner {
                    conditioner_type: "Brush".into(),
                    sweep_speed_rpm: 10.0,
                    down_force_lbs: 2.0,
                    in_situ: false,
                },
            },
        ],
        post_clean_recipe: "MEGASONIC-DIW-V3".into(),
    };
    let encoded = encode_to_vec(&cmp).expect("encode cmp process");
    let (decoded, _): (CmpProcess, _) = decode_from_slice(&encoded).expect("decode cmp process");
    assert_eq!(cmp, decoded);
}

#[test]
fn test_ion_implant_recipe() {
    let recipe = IonImplantRecipe {
        recipe_id: "IMPL-NWELL-V5".into(),
        tool_name: "AMAT-VIISta-900".into(),
        profiles: vec![
            ImplantProfile {
                profile_name: "N-Well Deep".into(),
                species: ImplantSpecies {
                    element: "P".into(),
                    isotope_mass: 31,
                    charge_state: 1,
                },
                dose_params: ImplantDose {
                    dose_atoms_per_cm2: 1.5e13,
                    energy_kev: 500.0,
                    tilt_deg: 7.0,
                    twist_deg: 22.0,
                },
                beam_current_ua: 800.0,
                scan_mode: "hybrid".into(),
            },
            ImplantProfile {
                profile_name: "N-Well Retro".into(),
                species: ImplantSpecies {
                    element: "P".into(),
                    isotope_mass: 31,
                    charge_state: 2,
                },
                dose_params: ImplantDose {
                    dose_atoms_per_cm2: 5.0e12,
                    energy_kev: 180.0,
                    tilt_deg: 7.0,
                    twist_deg: 22.0,
                },
                beam_current_ua: 500.0,
                scan_mode: "parallel".into(),
            },
        ],
        anneal_required: true,
        anneal_temp_c: Some(1050.0),
    };
    let encoded = encode_to_vec(&recipe).expect("encode implant recipe");
    let (decoded, _): (IonImplantRecipe, _) =
        decode_from_slice(&encoded).expect("decode implant recipe");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_metrology_report_cd_sem() {
    let report = MetrologyReport {
        wafer_id: "W01-N7-0315".into(),
        step_name: "POST-ETCH-M1".into(),
        cd_sem: vec![
            CdSemMeasurement {
                feature_name: "M1_LINE_H".into(),
                target_cd_nm: 28.0,
                measured_cd_nm: 27.6,
                lwr_nm: 1.8,
                lwe_nm: 1.2,
            },
            CdSemMeasurement {
                feature_name: "M1_SPACE_V".into(),
                target_cd_nm: 28.0,
                measured_cd_nm: 28.3,
                lwr_nm: 2.0,
                lwe_nm: 1.5,
            },
        ],
        ocd: vec![OcdMeasurement {
            profile_name: "M1_TRENCH".into(),
            cd_top_nm: 29.5,
            cd_bottom_nm: 26.0,
            sidewall_angle_deg: 86.5,
            height_nm: 100.0,
            goodness_of_fit: 0.998,
        }],
        film: vec![],
        timestamp_epoch: 1710489600,
    };
    let encoded = encode_to_vec(&report).expect("encode metrology report");
    let (decoded, _): (MetrologyReport, _) =
        decode_from_slice(&encoded).expect("decode metrology report");
    assert_eq!(report, decoded);
}

#[test]
fn test_metrology_film_thickness() {
    let report = MetrologyReport {
        wafer_id: "W12-N5-0410".into(),
        step_name: "POST-DEP-ILD".into(),
        cd_sem: vec![],
        ocd: vec![],
        film: vec![
            FilmThickness {
                film_type: "SiO2-PECVD".into(),
                target_nm: 500.0,
                measured_nm: 498.3,
                uniformity_pct: 1.2,
                measurement_points: 49,
            },
            FilmThickness {
                film_type: "SiN-Cap".into(),
                target_nm: 30.0,
                measured_nm: 30.8,
                uniformity_pct: 2.1,
                measurement_points: 49,
            },
        ],
        timestamp_epoch: 1710576000,
    };
    let encoded = encode_to_vec(&report).expect("encode film report");
    let (decoded, _): (MetrologyReport, _) =
        decode_from_slice(&encoded).expect("decode film report");
    assert_eq!(report, decoded);
}

#[test]
fn test_yield_analysis_wafer_map() {
    let yield_rpt = YieldReport {
        lot_id: "LOT-2026-0315-B".into(),
        wafer_maps: vec![WaferMap {
            wafer_id: "W03".into(),
            die_results: vec![
                DieBinResult {
                    die: DieCoordinate {
                        row: 5,
                        col: 10,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 450,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 5,
                        col: 11,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 460,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 6,
                        col: 10,
                        site: 1,
                    },
                    hard_bin: 5,
                    soft_bin: 510,
                    test_time_ms: 380,
                },
            ],
            total_die: 500,
            good_die: 480,
            yield_pct: 96.0,
        }],
        bin_definitions: vec![
            BinCode {
                bin_number: 1,
                bin_name: "PASS".into(),
                pass_fail: true,
                count: 480,
            },
            BinCode {
                bin_number: 5,
                bin_name: "FAIL-LEAK".into(),
                pass_fail: false,
                count: 20,
            },
        ],
        lot_yield_pct: 96.0,
    };
    let encoded = encode_to_vec(&yield_rpt).expect("encode yield report");
    let (decoded, _): (YieldReport, _) = decode_from_slice(&encoded).expect("decode yield report");
    assert_eq!(yield_rpt, decoded);
}

#[test]
fn test_clean_room_environment() {
    let env = FabEnvironment {
        fab_name: "FAB-12-HSINCHU".into(),
        zones: vec![
            CleanRoomZone {
                zone_name: "LITHO-BAY".into(),
                iso_class: 3,
                environment: EnvironmentReading {
                    temperature_c: 22.0,
                    humidity_pct: 43.0,
                    pressure_pa: 101425.0,
                    airflow_m_per_s: 0.45,
                },
                particle_counts: vec![
                    ParticleCount {
                        size_threshold_um: 0.1,
                        count_per_cubic_ft: 35,
                        measurement_location: "Above-Scanner".into(),
                    },
                    ParticleCount {
                        size_threshold_um: 0.5,
                        count_per_cubic_ft: 2,
                        measurement_location: "Track-Load".into(),
                    },
                ],
                personnel_count: 3,
            },
            CleanRoomZone {
                zone_name: "DIFFUSION-BAY".into(),
                iso_class: 5,
                environment: EnvironmentReading {
                    temperature_c: 22.5,
                    humidity_pct: 44.0,
                    pressure_pa: 101400.0,
                    airflow_m_per_s: 0.35,
                },
                particle_counts: vec![ParticleCount {
                    size_threshold_um: 0.5,
                    count_per_cubic_ft: 80,
                    measurement_location: "Furnace-Front".into(),
                }],
                personnel_count: 2,
            },
        ],
        total_area_sqm: 12000.0,
        monitoring_interval_sec: 300,
    };
    let encoded = encode_to_vec(&env).expect("encode fab environment");
    let (decoded, _): (FabEnvironment, _) =
        decode_from_slice(&encoded).expect("decode fab environment");
    assert_eq!(env, decoded);
}

#[test]
fn test_diffusion_furnace_recipe() {
    let run = DiffusionRun {
        run_id: "DIFF-20260315-001".into(),
        lot: LotInfo {
            lot_id: "LOT-GATE-OX".into(),
            product_code: "N5-HPC".into(),
            technology_node_nm: 5,
            wafer_count: 25,
            priority: 2,
        },
        recipe: FurnaceRecipe {
            recipe_name: "GATE-OX-12A".into(),
            tube_id: "TUBE-04".into(),
            ramp_segments: vec![
                FurnaceRamp {
                    segment_name: "Ramp-Up".into(),
                    start_temp_c: 600.0,
                    end_temp_c: 850.0,
                    ramp_rate_c_per_min: 5.0,
                    hold_time_min: 0.0,
                },
                FurnaceRamp {
                    segment_name: "Oxidation".into(),
                    start_temp_c: 850.0,
                    end_temp_c: 850.0,
                    ramp_rate_c_per_min: 0.0,
                    hold_time_min: 12.0,
                },
                FurnaceRamp {
                    segment_name: "Anneal".into(),
                    start_temp_c: 850.0,
                    end_temp_c: 1000.0,
                    ramp_rate_c_per_min: 3.0,
                    hold_time_min: 5.0,
                },
                FurnaceRamp {
                    segment_name: "Cool-Down".into(),
                    start_temp_c: 1000.0,
                    end_temp_c: 600.0,
                    ramp_rate_c_per_min: -2.0,
                    hold_time_min: 0.0,
                },
            ],
            gas_profile: vec![
                FurnaceGas {
                    gas_name: "N2".into(),
                    flow_slm: 10.0,
                    start_time_min: 0.0,
                    stop_time_min: 50.0,
                },
                FurnaceGas {
                    gas_name: "O2-DRY".into(),
                    flow_slm: 5.0,
                    start_time_min: 50.0,
                    stop_time_min: 62.0,
                },
                FurnaceGas {
                    gas_name: "N2O".into(),
                    flow_slm: 2.0,
                    start_time_min: 62.0,
                    stop_time_min: 67.0,
                },
            ],
            boat_rotation_rpm: 1.5,
        },
        oxide_target_nm: 1.2,
        oxide_measured_nm: Some(1.18),
    };
    let encoded = encode_to_vec(&run).expect("encode diffusion run");
    let (decoded, _): (DiffusionRun, _) =
        decode_from_slice(&encoded).expect("decode diffusion run");
    assert_eq!(run, decoded);
}

#[test]
fn test_cvd_deposition_process() {
    let dep = DepositionProcess {
        process_id: "CVD-ILD-V6".into(),
        tool_id: "AMAT-PRODUCER-07".into(),
        layers: vec![
            DepositionLayer {
                layer_name: "USG-Liner".into(),
                method: "PECVD".into(),
                precursors: vec![
                    PrecursorGas {
                        chemical_name: "TEOS".into(),
                        flow_sccm: 1200.0,
                        bubbler_temp_c: Some(35.0),
                        carrier_gas: Some("He".into()),
                    },
                    PrecursorGas {
                        chemical_name: "O2".into(),
                        flow_sccm: 600.0,
                        bubbler_temp_c: None,
                        carrier_gas: None,
                    },
                ],
                conditions: DepositionConditions {
                    temperature_c: 400.0,
                    pressure_torr: 8.0,
                    rf_power_watts: Some(700.0),
                    dc_power_kw: None,
                    substrate_bias_v: None,
                },
                target_thickness_nm: 50.0,
                deposition_rate_nm_per_min: 250.0,
            },
            DepositionLayer {
                layer_name: "Low-k-ILD".into(),
                method: "PECVD".into(),
                precursors: vec![PrecursorGas {
                    chemical_name: "DEMS".into(),
                    flow_sccm: 800.0,
                    bubbler_temp_c: Some(40.0),
                    carrier_gas: Some("He".into()),
                }],
                conditions: DepositionConditions {
                    temperature_c: 350.0,
                    pressure_torr: 6.0,
                    rf_power_watts: Some(500.0),
                    dc_power_kw: None,
                    substrate_bias_v: Some(-50.0),
                },
                target_thickness_nm: 200.0,
                deposition_rate_nm_per_min: 150.0,
            },
        ],
        total_time_min: 3.5,
    };
    let encoded = encode_to_vec(&dep).expect("encode deposition process");
    let (decoded, _): (DepositionProcess, _) =
        decode_from_slice(&encoded).expect("decode deposition process");
    assert_eq!(dep, decoded);
}

#[test]
fn test_pvd_sputtering_layer() {
    let dep = DepositionProcess {
        process_id: "PVD-BARRIER-TaN".into(),
        tool_id: "AMAT-ENDURA-03".into(),
        layers: vec![DepositionLayer {
            layer_name: "TaN-Barrier".into(),
            method: "PVD-Reactive".into(),
            precursors: vec![PrecursorGas {
                chemical_name: "N2".into(),
                flow_sccm: 30.0,
                bubbler_temp_c: None,
                carrier_gas: None,
            }],
            conditions: DepositionConditions {
                temperature_c: 250.0,
                pressure_torr: 0.003,
                rf_power_watts: None,
                dc_power_kw: Some(12.0),
                substrate_bias_v: Some(-100.0),
            },
            target_thickness_nm: 3.0,
            deposition_rate_nm_per_min: 15.0,
        }],
        total_time_min: 0.2,
    };
    let encoded = encode_to_vec(&dep).expect("encode pvd process");
    let (decoded, _): (DepositionProcess, _) =
        decode_from_slice(&encoded).expect("decode pvd process");
    assert_eq!(dep, decoded);
}

#[test]
fn test_electrical_test_transistor_params() {
    let etest = EtestWafer {
        wafer_id: "W07-N7-ETEST".into(),
        sites: vec![
            EtestSite {
                site_id: 1,
                die: DieCoordinate {
                    row: 12,
                    col: 15,
                    site: 1,
                },
                transistors: vec![
                    TransistorParams {
                        device_type: "NFET".into(),
                        channel_length_nm: 7.0,
                        channel_width_nm: 100.0,
                        vt_mv: 280.0,
                        idsat_ua_per_um: 1050.0,
                        ioff_pa_per_um: 5.0,
                        subthreshold_swing_mv_per_dec: 68.0,
                    },
                    TransistorParams {
                        device_type: "PFET".into(),
                        channel_length_nm: 7.0,
                        channel_width_nm: 100.0,
                        vt_mv: -310.0,
                        idsat_ua_per_um: 780.0,
                        ioff_pa_per_um: 3.0,
                        subthreshold_swing_mv_per_dec: 70.0,
                    },
                ],
                leakage: vec![LeakageCurrent {
                    junction_name: "N+/P-Well".into(),
                    leakage_na: 0.12,
                    voltage_v: -1.0,
                    temperature_c: 25.0,
                }],
            },
            EtestSite {
                site_id: 2,
                die: DieCoordinate {
                    row: 12,
                    col: 16,
                    site: 1,
                },
                transistors: vec![TransistorParams {
                    device_type: "NFET".into(),
                    channel_length_nm: 7.0,
                    channel_width_nm: 100.0,
                    vt_mv: 275.0,
                    idsat_ua_per_um: 1060.0,
                    ioff_pa_per_um: 4.8,
                    subthreshold_swing_mv_per_dec: 67.5,
                }],
                leakage: vec![],
            },
        ],
        pass: true,
        median_vt_mv: 278.0,
    };
    let encoded = encode_to_vec(&etest).expect("encode etest wafer");
    let (decoded, _): (EtestWafer, _) = decode_from_slice(&encoded).expect("decode etest wafer");
    assert_eq!(etest, decoded);
}

#[test]
fn test_wire_bond_packaging() {
    let pkg = PackageDesign {
        package_type: "QFN-48".into(),
        body_size_mm_x: 7.0,
        body_size_mm_y: 7.0,
        substrate: PackageSubstrate {
            substrate_type: "Laminate".into(),
            layer_count: 4,
            thickness_mm: 0.4,
            material: "BT-Resin".into(),
        },
        wire_bonds: vec![
            WireBondSpec {
                pad_name: "VDD".into(),
                wire_material: "Au".into(),
                wire_diameter_um: 25.0,
                loop_height_um: 200.0,
                ball_size_um: 55.0,
                bond_force_gf: 30.0,
            },
            WireBondSpec {
                pad_name: "IO_0".into(),
                wire_material: "Au".into(),
                wire_diameter_um: 20.0,
                loop_height_um: 180.0,
                ball_size_um: 48.0,
                bond_force_gf: 25.0,
            },
        ],
        flip_chip_bumps: vec![],
        lead_count: 48,
    };
    let encoded = encode_to_vec(&pkg).expect("encode wire bond package");
    let (decoded, _): (PackageDesign, _) =
        decode_from_slice(&encoded).expect("decode wire bond package");
    assert_eq!(pkg, decoded);
}

#[test]
fn test_flip_chip_packaging() {
    let pkg = PackageDesign {
        package_type: "FCBGA-1024".into(),
        body_size_mm_x: 35.0,
        body_size_mm_y: 35.0,
        substrate: PackageSubstrate {
            substrate_type: "ABF-Buildup".into(),
            layer_count: 12,
            thickness_mm: 1.2,
            material: "ABF-GX92".into(),
        },
        wire_bonds: vec![],
        flip_chip_bumps: vec![
            FlipChipBump {
                bump_id: "C4-A1".into(),
                material: "SnAg".into(),
                diameter_um: 80.0,
                pitch_um: 150.0,
                height_um: 50.0,
                underfill: true,
            },
            FlipChipBump {
                bump_id: "C4-A2".into(),
                material: "SnAg".into(),
                diameter_um: 80.0,
                pitch_um: 150.0,
                height_um: 50.0,
                underfill: true,
            },
            FlipChipBump {
                bump_id: "uBump-B1".into(),
                material: "Cu-Pillar".into(),
                diameter_um: 25.0,
                pitch_um: 40.0,
                height_um: 30.0,
                underfill: true,
            },
        ],
        lead_count: 1024,
    };
    let encoded = encode_to_vec(&pkg).expect("encode flip chip package");
    let (decoded, _): (PackageDesign, _) =
        decode_from_slice(&encoded).expect("decode flip chip package");
    assert_eq!(pkg, decoded);
}

#[test]
fn test_esd_protection_spec() {
    let esd = EsdSpec {
        device_name: "APEX7-IO-PAD".into(),
        protection_cells: vec![
            EsdProtection {
                clamp_type: "GGNMOS".into(),
                trigger_voltage_v: 6.5,
                holding_voltage_v: 3.5,
                on_resistance_ohm: 2.5,
            },
            EsdProtection {
                clamp_type: "RC-Triggered-Clamp".into(),
                trigger_voltage_v: 5.0,
                holding_voltage_v: 1.8,
                on_resistance_ohm: 1.0,
            },
        ],
        test_results: vec![
            EsdTestResult {
                model: "HBM".into(),
                voltage_kv: 2.0,
                pin_name: "IO_5".into(),
                passed: true,
                leakage_after_na: 0.8,
            },
            EsdTestResult {
                model: "CDM".into(),
                voltage_kv: 0.5,
                pin_name: "IO_5".into(),
                passed: true,
                leakage_after_na: 1.2,
            },
            EsdTestResult {
                model: "HBM".into(),
                voltage_kv: 2.0,
                pin_name: "VDD".into(),
                passed: true,
                leakage_after_na: 0.5,
            },
        ],
        classification: "Class-2".into(),
    };
    let encoded = encode_to_vec(&esd).expect("encode esd spec");
    let (decoded, _): (EsdSpec, _) = decode_from_slice(&encoded).expect("decode esd spec");
    assert_eq!(esd, decoded);
}

#[test]
fn test_defect_inspection_report() {
    let report = DefectInspectionReport {
        wafer_id: "W19-N7-INSP".into(),
        tool_id: "KLA-2925-01".into(),
        recipe_name: "M1-POST-ETCH-BRIGHT".into(),
        defects: vec![
            DefectEntry {
                defect_id: 10001,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 8,
                        col: 12,
                        site: 1,
                    },
                    x_um: 1523.4,
                    y_um: 2801.7,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.08,
                    aspect_ratio: 1.2,
                    brightness: 180.0,
                    polarity: "BRIGHT".into(),
                },
                classification: "BRIDGE".into(),
                killer: true,
                review_completed: true,
            },
            DefectEntry {
                defect_id: 10002,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 9,
                        col: 14,
                        site: 1,
                    },
                    x_um: 3100.5,
                    y_um: 450.2,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.15,
                    aspect_ratio: 3.5,
                    brightness: 50.0,
                    polarity: "DARK".into(),
                },
                classification: "OPEN".into(),
                killer: true,
                review_completed: true,
            },
            DefectEntry {
                defect_id: 10003,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 3,
                        col: 5,
                        site: 1,
                    },
                    x_um: 800.0,
                    y_um: 1200.0,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.04,
                    aspect_ratio: 1.0,
                    brightness: 120.0,
                    polarity: "BRIGHT".into(),
                },
                classification: "PARTICLE".into(),
                killer: false,
                review_completed: false,
            },
        ],
        total_defects: 3,
        killer_defects: 2,
        defect_density_per_cm2: 0.15,
    };
    let encoded = encode_to_vec(&report).expect("encode defect report");
    let (decoded, _): (DefectInspectionReport, _) =
        decode_from_slice(&encoded).expect("decode defect report");
    assert_eq!(report, decoded);
}

#[test]
fn test_implant_without_anneal() {
    let recipe = IonImplantRecipe {
        recipe_id: "IMPL-HALO-V2".into(),
        tool_name: "AXCELIS-PURION-H".into(),
        profiles: vec![ImplantProfile {
            profile_name: "Halo-NMOS".into(),
            species: ImplantSpecies {
                element: "In".into(),
                isotope_mass: 115,
                charge_state: 1,
            },
            dose_params: ImplantDose {
                dose_atoms_per_cm2: 3.0e13,
                energy_kev: 60.0,
                tilt_deg: 28.0,
                twist_deg: 0.0,
            },
            beam_current_ua: 200.0,
            scan_mode: "quad".into(),
        }],
        anneal_required: false,
        anneal_temp_c: None,
    };
    let encoded = encode_to_vec(&recipe).expect("encode halo implant");
    let (decoded, _): (IonImplantRecipe, _) =
        decode_from_slice(&encoded).expect("decode halo implant");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_multi_wafer_yield_report() {
    let make_map = |id: &str, good: u32, total: u32| -> WaferMap {
        WaferMap {
            wafer_id: id.into(),
            die_results: vec![
                DieBinResult {
                    die: DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 500,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 1,
                    },
                    hard_bin: 2,
                    soft_bin: 200,
                    test_time_ms: 520,
                },
            ],
            total_die: total,
            good_die: good,
            yield_pct: (good as f64 / total as f64) * 100.0,
        }
    };

    let report = YieldReport {
        lot_id: "LOT-MULTI-YIELD".into(),
        wafer_maps: vec![
            make_map("W01", 490, 500),
            make_map("W02", 485, 500),
            make_map("W03", 495, 500),
        ],
        bin_definitions: vec![
            BinCode {
                bin_number: 1,
                bin_name: "PASS".into(),
                pass_fail: true,
                count: 1470,
            },
            BinCode {
                bin_number: 2,
                bin_name: "FAIL-VT".into(),
                pass_fail: false,
                count: 30,
            },
        ],
        lot_yield_pct: 98.0,
    };
    let encoded = encode_to_vec(&report).expect("encode multi-wafer yield");
    let (decoded, _): (YieldReport, _) =
        decode_from_slice(&encoded).expect("decode multi-wafer yield");
    assert_eq!(report, decoded);
}

#[test]
fn test_overlay_tight_tolerance() {
    let litho = LithoStep {
        step_name: "VIA1_PHOTO".into(),
        tool_id: "ASML-NXE3400-02".into(),
        reticle: ReticleInfo {
            reticle_id: "RTL-V1-V1R1".into(),
            layer_name: "VIA1".into(),
            field_size_x_mm: 26.0,
            field_size_y_mm: 33.0,
            pellicle_attached: true,
        },
        exposure: ExposureSettings {
            dose_mj_per_cm2: 33.0,
            focus_offset_nm: -5.0,
            numerical_aperture: 0.33,
            sigma_inner: 0.5,
            sigma_outer: 0.85,
            wavelength_nm: 13.5,
        },
        overlay: OverlayError {
            x_offset_nm: 0.15,
            y_offset_nm: -0.08,
            rotation_urad: 0.02,
            magnification_ppm: 0.01,
        },
        alignment_marks_used: 16,
    };
    let encoded = encode_to_vec(&litho).expect("encode overlay litho");
    let (decoded, _): (LithoStep, _) = decode_from_slice(&encoded).expect("decode overlay litho");
    assert_eq!(litho, decoded);
}

#[test]
fn test_diffusion_no_measurement() {
    let run = DiffusionRun {
        run_id: "DIFF-20260315-005".into(),
        lot: LotInfo {
            lot_id: "LOT-LINER-OX".into(),
            product_code: "N3-HVM".into(),
            technology_node_nm: 3,
            wafer_count: 13,
            priority: 3,
        },
        recipe: FurnaceRecipe {
            recipe_name: "LINER-OX-5A".into(),
            tube_id: "TUBE-09".into(),
            ramp_segments: vec![FurnaceRamp {
                segment_name: "Rapid-Ox".into(),
                start_temp_c: 700.0,
                end_temp_c: 700.0,
                ramp_rate_c_per_min: 0.0,
                hold_time_min: 2.0,
            }],
            gas_profile: vec![FurnaceGas {
                gas_name: "O2-DRY".into(),
                flow_slm: 3.0,
                start_time_min: 0.0,
                stop_time_min: 2.0,
            }],
            boat_rotation_rpm: 2.0,
        },
        oxide_target_nm: 0.5,
        oxide_measured_nm: None,
    };
    let encoded = encode_to_vec(&run).expect("encode liner ox run");
    let (decoded, _): (DiffusionRun, _) = decode_from_slice(&encoded).expect("decode liner ox run");
    assert_eq!(run, decoded);
}

#[test]
fn test_empty_defect_report() {
    let report = DefectInspectionReport {
        wafer_id: "W25-CLEAN".into(),
        tool_id: "KLA-2925-02".into(),
        recipe_name: "BARE-SI-BASELINE".into(),
        defects: vec![],
        total_defects: 0,
        killer_defects: 0,
        defect_density_per_cm2: 0.0,
    };
    let encoded = encode_to_vec(&report).expect("encode clean defect report");
    let (decoded, _): (DefectInspectionReport, _) =
        decode_from_slice(&encoded).expect("decode clean defect report");
    assert_eq!(report, decoded);
}
