//! Advanced nested structs test — additive manufacturing and 3D printing theme, 22 tests.

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
// Domain types — Level 0 (leaf / simple)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoundingBox {
    min: Vertex,
    max: Vertex,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InfillPattern {
    Grid,
    Gyroid,
    Honeycomb,
    Lines,
    Triangles,
    Cubic,
    Lightning,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FilamentMaterial {
    Pla,
    Abs,
    Petg,
    Nylon,
    Tpu,
    Resin,
    CarbonFiber,
    Asa,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureProfile {
    nozzle_temp_c: u16,
    bed_temp_c: u16,
    chamber_temp_c: Option<u16>,
    first_layer_nozzle_offset: i8,
    first_layer_bed_offset: i8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PidTuning {
    kp: f64,
    ki: f64,
    kd: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DimensionalMeasurement {
    axis_label: String,
    expected_mm: f64,
    measured_mm: f64,
    deviation_mm: f64,
}

// ---------------------------------------------------------------------------
// Domain types — Level 1
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StlModelMetadata {
    filename: String,
    vertex_count: u64,
    triangle_count: u64,
    bounding_box: BoundingBox,
    file_size_bytes: u64,
    is_watertight: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SupportStructureConfig {
    enabled: bool,
    pattern: InfillPattern,
    density_percent: u8,
    overhang_angle_deg: u8,
    z_distance_mm: f64,
    xy_distance_mm: f64,
    interface_layers: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BuildPlateAdhesion {
    adhesion_type: String,
    brim_width_mm: f64,
    raft_layers: u8,
    raft_air_gap_mm: f64,
    skirt_line_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialProfile {
    material: FilamentMaterial,
    brand: String,
    color: String,
    diameter_mm: f64,
    density_g_per_cm3: f64,
    temperature: TemperatureProfile,
    max_volumetric_speed: f64,
    cost_per_kg_usd: f64,
    spool_weight_g: u32,
    remaining_weight_g: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BedLevelingMesh {
    rows: u8,
    cols: u8,
    probe_points: Vec<f64>,
    z_offset_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EStepsCalibration {
    extruder_index: u8,
    steps_per_mm: f64,
    requested_mm: f64,
    actual_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PostProcessingStep {
    step_name: String,
    duration_minutes: u32,
    temperature_c: Option<u16>,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TensileTestResult {
    specimen_id: String,
    ultimate_strength_mpa: f64,
    yield_strength_mpa: f64,
    elongation_percent: f64,
    layer_orientation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CostLineItem {
    description: String,
    quantity: f64,
    unit_cost_usd: f64,
    total_usd: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TopologyRegion {
    region_id: u32,
    density_fraction: f64,
    volume_mm3: f64,
    stress_mpa: f64,
    retained: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScaffoldPore {
    pore_diameter_um: f64,
    strut_thickness_um: f64,
    porosity_percent: f64,
    geometry: String,
}

// ---------------------------------------------------------------------------
// Domain types — Level 2
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrintJobConfig {
    job_name: String,
    model: StlModelMetadata,
    layer_height_mm: f64,
    first_layer_height_mm: f64,
    infill_pattern: InfillPattern,
    infill_density_percent: u8,
    wall_line_count: u8,
    top_layers: u8,
    bottom_layers: u8,
    support: SupportStructureConfig,
    adhesion: BuildPlateAdhesion,
    print_speed_mm_s: f64,
    travel_speed_mm_s: f64,
    retraction_distance_mm: f64,
    retraction_speed_mm_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrinterCalibration {
    printer_name: String,
    firmware_version: String,
    bed_leveling: BedLevelingMesh,
    e_steps: Vec<EStepsCalibration>,
    hotend_pid: PidTuning,
    bed_pid: PidTuning,
    max_acceleration_mm_s2: u32,
    max_jerk_mm_s: f64,
    calibrated_at: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GcodeSettings {
    flavor: String,
    start_gcode: String,
    end_gcode: String,
    layer_change_gcode: String,
    enable_arc_welder: bool,
    resolution_mm: f64,
    max_gcode_per_second: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MultiMaterialConfig {
    tool_count: u8,
    materials: Vec<MaterialProfile>,
    purge_tower_enabled: bool,
    purge_volume_mm3: f64,
    wipe_tower_x: f64,
    wipe_tower_y: f64,
    tool_change_retraction_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PostProcessingPipeline {
    steps: Vec<PostProcessingStep>,
    total_time_minutes: u32,
    requires_ventilation: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityControlReport {
    part_id: String,
    surface_roughness_ra_um: f64,
    dimensional_checks: Vec<DimensionalMeasurement>,
    tensile_tests: Vec<TensileTestResult>,
    visual_pass: bool,
    inspector: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CostEstimation {
    job_name: String,
    line_items: Vec<CostLineItem>,
    subtotal_usd: f64,
    markup_percent: f64,
    total_usd: f64,
    estimated_print_hours: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TopologyOptimizationResult {
    iteration_count: u32,
    original_volume_mm3: f64,
    optimized_volume_mm3: f64,
    volume_reduction_percent: f64,
    regions: Vec<TopologyRegion>,
    compliance: f64,
    converged: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BioprintingScaffold {
    scaffold_name: String,
    pore: ScaffoldPore,
    layer_count: u32,
    bio_ink_material: String,
    crosslinking_method: String,
    cell_seeding_density: f64,
    target_tissue: String,
    degradation_weeks: u16,
}

// ---------------------------------------------------------------------------
// Domain types — Level 3 (deeply nested)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrintFarmJob {
    printer_id: String,
    calibration: PrinterCalibration,
    job_config: PrintJobConfig,
    material: MaterialProfile,
    gcode: GcodeSettings,
    estimated_time_minutes: u32,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrintFarmStatus {
    farm_name: String,
    location: String,
    active_jobs: Vec<PrintFarmJob>,
    queued_job_count: u32,
    total_printers: u16,
    online_printers: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FullPartReport {
    order_id: String,
    job: PrintJobConfig,
    post_processing: PostProcessingPipeline,
    quality: QualityControlReport,
    cost: CostEstimation,
    shipped: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MultiMaterialPrintSession {
    session_id: String,
    printer_calibration: PrinterCalibration,
    multi_material: MultiMaterialConfig,
    job: PrintJobConfig,
    gcode: GcodeSettings,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BioprintingProject {
    project_name: String,
    scaffold: BioprintingScaffold,
    topology: TopologyOptimizationResult,
    quality: QualityControlReport,
    cost: CostEstimation,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sample_vertex(x: f64, y: f64, z: f64) -> Vertex {
    Vertex { x, y, z }
}

fn sample_bounding_box() -> BoundingBox {
    BoundingBox {
        min: sample_vertex(0.0, 0.0, 0.0),
        max: sample_vertex(100.0, 80.0, 60.0),
    }
}

fn sample_temperature_profile(material: &FilamentMaterial) -> TemperatureProfile {
    match material {
        FilamentMaterial::Pla => TemperatureProfile {
            nozzle_temp_c: 210,
            bed_temp_c: 60,
            chamber_temp_c: None,
            first_layer_nozzle_offset: 5,
            first_layer_bed_offset: 5,
        },
        FilamentMaterial::Abs => TemperatureProfile {
            nozzle_temp_c: 240,
            bed_temp_c: 100,
            chamber_temp_c: Some(50),
            first_layer_nozzle_offset: 5,
            first_layer_bed_offset: 10,
        },
        FilamentMaterial::Petg => TemperatureProfile {
            nozzle_temp_c: 235,
            bed_temp_c: 80,
            chamber_temp_c: None,
            first_layer_nozzle_offset: 0,
            first_layer_bed_offset: 5,
        },
        FilamentMaterial::Nylon => TemperatureProfile {
            nozzle_temp_c: 260,
            bed_temp_c: 90,
            chamber_temp_c: Some(55),
            first_layer_nozzle_offset: 5,
            first_layer_bed_offset: 10,
        },
        _ => TemperatureProfile {
            nozzle_temp_c: 220,
            bed_temp_c: 65,
            chamber_temp_c: None,
            first_layer_nozzle_offset: 0,
            first_layer_bed_offset: 0,
        },
    }
}

fn sample_material(mat: FilamentMaterial, brand: &str, color: &str) -> MaterialProfile {
    let temp = sample_temperature_profile(&mat);
    MaterialProfile {
        material: mat,
        brand: brand.to_string(),
        color: color.to_string(),
        diameter_mm: 1.75,
        density_g_per_cm3: 1.24,
        temperature: temp,
        max_volumetric_speed: 15.0,
        cost_per_kg_usd: 25.0,
        spool_weight_g: 1000,
        remaining_weight_g: 750,
    }
}

fn sample_stl_model(name: &str) -> StlModelMetadata {
    StlModelMetadata {
        filename: format!("{}.stl", name),
        vertex_count: 48_000,
        triangle_count: 16_000,
        bounding_box: sample_bounding_box(),
        file_size_bytes: 800_000,
        is_watertight: true,
    }
}

fn sample_support_config() -> SupportStructureConfig {
    SupportStructureConfig {
        enabled: true,
        pattern: InfillPattern::Grid,
        density_percent: 15,
        overhang_angle_deg: 45,
        z_distance_mm: 0.2,
        xy_distance_mm: 0.7,
        interface_layers: 3,
    }
}

fn sample_adhesion() -> BuildPlateAdhesion {
    BuildPlateAdhesion {
        adhesion_type: "brim".to_string(),
        brim_width_mm: 8.0,
        raft_layers: 0,
        raft_air_gap_mm: 0.0,
        skirt_line_count: 3,
    }
}

fn sample_print_job(name: &str) -> PrintJobConfig {
    PrintJobConfig {
        job_name: name.to_string(),
        model: sample_stl_model(name),
        layer_height_mm: 0.2,
        first_layer_height_mm: 0.3,
        infill_pattern: InfillPattern::Gyroid,
        infill_density_percent: 20,
        wall_line_count: 3,
        top_layers: 5,
        bottom_layers: 4,
        support: sample_support_config(),
        adhesion: sample_adhesion(),
        print_speed_mm_s: 60.0,
        travel_speed_mm_s: 150.0,
        retraction_distance_mm: 0.8,
        retraction_speed_mm_s: 45.0,
    }
}

fn sample_bed_leveling() -> BedLevelingMesh {
    BedLevelingMesh {
        rows: 5,
        cols: 5,
        probe_points: vec![
            0.01, 0.02, -0.01, 0.00, 0.03, 0.02, 0.01, 0.00, -0.02, 0.01, -0.01, 0.00, 0.01, 0.02,
            0.00, 0.03, 0.01, -0.01, 0.00, 0.02, 0.00, -0.02, 0.01, 0.03, 0.01,
        ],
        z_offset_mm: -1.85,
    }
}

fn sample_pid() -> PidTuning {
    PidTuning {
        kp: 22.2,
        ki: 1.08,
        kd: 114.0,
    }
}

fn sample_e_steps(index: u8) -> EStepsCalibration {
    EStepsCalibration {
        extruder_index: index,
        steps_per_mm: 415.0,
        requested_mm: 100.0,
        actual_mm: 99.8,
    }
}

fn sample_printer_calibration(name: &str) -> PrinterCalibration {
    PrinterCalibration {
        printer_name: name.to_string(),
        firmware_version: "Marlin 2.1.2.4".to_string(),
        bed_leveling: sample_bed_leveling(),
        e_steps: vec![sample_e_steps(0)],
        hotend_pid: sample_pid(),
        bed_pid: PidTuning {
            kp: 60.0,
            ki: 0.65,
            kd: 750.0,
        },
        max_acceleration_mm_s2: 3000,
        max_jerk_mm_s: 8.0,
        calibrated_at: "2026-03-10T14:30:00Z".to_string(),
    }
}

fn sample_gcode_settings() -> GcodeSettings {
    GcodeSettings {
        flavor: "Marlin".to_string(),
        start_gcode: "G28\nG29\nM104 S{nozzle_temp}\nM190 S{bed_temp}".to_string(),
        end_gcode: "G91\nG1 E-2 F2700\nG1 Z10\nG90\nG28 X Y\nM84".to_string(),
        layer_change_gcode: ";LAYER_CHANGE".to_string(),
        enable_arc_welder: true,
        resolution_mm: 0.05,
        max_gcode_per_second: 1000,
    }
}

fn sample_post_processing() -> PostProcessingPipeline {
    PostProcessingPipeline {
        steps: vec![
            PostProcessingStep {
                step_name: "Support removal".to_string(),
                duration_minutes: 15,
                temperature_c: None,
                notes: "Flush cutters and needle-nose pliers".to_string(),
            },
            PostProcessingStep {
                step_name: "Sanding".to_string(),
                duration_minutes: 30,
                temperature_c: None,
                notes: "Progressive grit 120 -> 400 -> 800".to_string(),
            },
            PostProcessingStep {
                step_name: "Primer coat".to_string(),
                duration_minutes: 45,
                temperature_c: Some(22),
                notes: "Filler primer, two coats".to_string(),
            },
        ],
        total_time_minutes: 90,
        requires_ventilation: true,
    }
}

fn sample_quality_report(part_id: &str) -> QualityControlReport {
    QualityControlReport {
        part_id: part_id.to_string(),
        surface_roughness_ra_um: 3.2,
        dimensional_checks: vec![
            DimensionalMeasurement {
                axis_label: "X".to_string(),
                expected_mm: 50.0,
                measured_mm: 49.92,
                deviation_mm: -0.08,
            },
            DimensionalMeasurement {
                axis_label: "Y".to_string(),
                expected_mm: 30.0,
                measured_mm: 30.05,
                deviation_mm: 0.05,
            },
            DimensionalMeasurement {
                axis_label: "Z".to_string(),
                expected_mm: 20.0,
                measured_mm: 20.12,
                deviation_mm: 0.12,
            },
        ],
        tensile_tests: vec![TensileTestResult {
            specimen_id: format!("{}-T1", part_id),
            ultimate_strength_mpa: 52.0,
            yield_strength_mpa: 38.0,
            elongation_percent: 6.5,
            layer_orientation: "XY".to_string(),
        }],
        visual_pass: true,
        inspector: "QA-Bot".to_string(),
    }
}

fn sample_cost_estimation(name: &str) -> CostEstimation {
    CostEstimation {
        job_name: name.to_string(),
        line_items: vec![
            CostLineItem {
                description: "Filament PLA".to_string(),
                quantity: 0.120,
                unit_cost_usd: 25.0,
                total_usd: 3.0,
            },
            CostLineItem {
                description: "Machine time".to_string(),
                quantity: 4.5,
                unit_cost_usd: 2.0,
                total_usd: 9.0,
            },
            CostLineItem {
                description: "Post-processing labor".to_string(),
                quantity: 1.5,
                unit_cost_usd: 15.0,
                total_usd: 22.5,
            },
        ],
        subtotal_usd: 34.5,
        markup_percent: 30.0,
        total_usd: 44.85,
        estimated_print_hours: 4.5,
    }
}

fn sample_topology_result() -> TopologyOptimizationResult {
    TopologyOptimizationResult {
        iteration_count: 150,
        original_volume_mm3: 50_000.0,
        optimized_volume_mm3: 28_000.0,
        volume_reduction_percent: 44.0,
        regions: vec![
            TopologyRegion {
                region_id: 1,
                density_fraction: 1.0,
                volume_mm3: 20_000.0,
                stress_mpa: 45.0,
                retained: true,
            },
            TopologyRegion {
                region_id: 2,
                density_fraction: 0.6,
                volume_mm3: 8_000.0,
                stress_mpa: 22.0,
                retained: true,
            },
            TopologyRegion {
                region_id: 3,
                density_fraction: 0.1,
                volume_mm3: 22_000.0,
                stress_mpa: 2.0,
                retained: false,
            },
        ],
        compliance: 0.0032,
        converged: true,
    }
}

fn sample_scaffold() -> BioprintingScaffold {
    BioprintingScaffold {
        scaffold_name: "Bone-scaffold-v3".to_string(),
        pore: ScaffoldPore {
            pore_diameter_um: 300.0,
            strut_thickness_um: 150.0,
            porosity_percent: 65.0,
            geometry: "gyroid".to_string(),
        },
        layer_count: 40,
        bio_ink_material: "GelMA-alginate".to_string(),
        crosslinking_method: "UV-405nm".to_string(),
        cell_seeding_density: 1e6,
        target_tissue: "trabecular bone".to_string(),
        degradation_weeks: 12,
    }
}

fn sample_multi_material_config() -> MultiMaterialConfig {
    MultiMaterialConfig {
        tool_count: 2,
        materials: vec![
            sample_material(FilamentMaterial::Pla, "eSUN", "White"),
            sample_material(FilamentMaterial::Petg, "Prusament", "Galaxy Black"),
        ],
        purge_tower_enabled: true,
        purge_volume_mm3: 60.0,
        wipe_tower_x: 170.0,
        wipe_tower_y: 140.0,
        tool_change_retraction_mm: 1.5,
    }
}

fn sample_print_farm_job(printer_id: &str, job_name: &str) -> PrintFarmJob {
    PrintFarmJob {
        printer_id: printer_id.to_string(),
        calibration: sample_printer_calibration(printer_id),
        job_config: sample_print_job(job_name),
        material: sample_material(FilamentMaterial::Pla, "eSUN", "Grey"),
        gcode: sample_gcode_settings(),
        estimated_time_minutes: 270,
        priority: 5,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Vertex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vertex_roundtrip() {
    let val = sample_vertex(12.5, -3.14, 100.001);
    let bytes = encode_to_vec(&val).expect("encode vertex");
    let (decoded, _): (Vertex, usize) = decode_from_slice(&bytes).expect("decode vertex");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: BoundingBox roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bounding_box_roundtrip() {
    let val = sample_bounding_box();
    let bytes = encode_to_vec(&val).expect("encode bounding box");
    let (decoded, _): (BoundingBox, usize) =
        decode_from_slice(&bytes).expect("decode bounding box");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: STL model metadata with nested bounding box
// ---------------------------------------------------------------------------

#[test]
fn test_stl_model_metadata_roundtrip() {
    let val = sample_stl_model("bracket_v2");
    let bytes = encode_to_vec(&val).expect("encode stl model");
    let (decoded, _): (StlModelMetadata, usize) =
        decode_from_slice(&bytes).expect("decode stl model");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Material profile with temperature settings
// ---------------------------------------------------------------------------

#[test]
fn test_material_profile_pla_roundtrip() {
    let val = sample_material(FilamentMaterial::Pla, "Prusament", "Pristine White");
    let bytes = encode_to_vec(&val).expect("encode PLA material");
    let (decoded, _): (MaterialProfile, usize) =
        decode_from_slice(&bytes).expect("decode PLA material");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: ABS material with chamber temperature
// ---------------------------------------------------------------------------

#[test]
fn test_material_profile_abs_with_chamber_temp() {
    let val = sample_material(FilamentMaterial::Abs, "Hatchbox", "Black");
    assert!(val.temperature.chamber_temp_c.is_some());
    let bytes = encode_to_vec(&val).expect("encode ABS material");
    let (decoded, _): (MaterialProfile, usize) =
        decode_from_slice(&bytes).expect("decode ABS material");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Support structure config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_support_structure_config_roundtrip() {
    let val = sample_support_config();
    let bytes = encode_to_vec(&val).expect("encode support config");
    let (decoded, _): (SupportStructureConfig, usize) =
        decode_from_slice(&bytes).expect("decode support config");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Print job config (deeply nested: job -> model -> bounding_box -> vertex)
// ---------------------------------------------------------------------------

#[test]
fn test_print_job_config_deep_nesting() {
    let val = sample_print_job("gear_housing");
    let bytes = encode_to_vec(&val).expect("encode print job");
    let (decoded, _): (PrintJobConfig, usize) =
        decode_from_slice(&bytes).expect("decode print job");
    assert_eq!(val, decoded);
    assert_eq!(decoded.model.bounding_box.min.x, 0.0);
    assert_eq!(decoded.support.pattern, InfillPattern::Grid);
}

// ---------------------------------------------------------------------------
// Test 8: Printer calibration with bed leveling mesh
// ---------------------------------------------------------------------------

#[test]
fn test_printer_calibration_roundtrip() {
    let val = sample_printer_calibration("Voron-2.4-350");
    let bytes = encode_to_vec(&val).expect("encode printer calibration");
    let (decoded, _): (PrinterCalibration, usize) =
        decode_from_slice(&bytes).expect("decode printer calibration");
    assert_eq!(val, decoded);
    assert_eq!(decoded.bed_leveling.probe_points.len(), 25);
}

// ---------------------------------------------------------------------------
// Test 9: G-code generation settings
// ---------------------------------------------------------------------------

#[test]
fn test_gcode_settings_roundtrip() {
    let val = sample_gcode_settings();
    let bytes = encode_to_vec(&val).expect("encode gcode settings");
    let (decoded, _): (GcodeSettings, usize) =
        decode_from_slice(&bytes).expect("decode gcode settings");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Multi-material config with multiple material profiles
// ---------------------------------------------------------------------------

#[test]
fn test_multi_material_config_roundtrip() {
    let val = sample_multi_material_config();
    let bytes = encode_to_vec(&val).expect("encode multi-material config");
    let (decoded, _): (MultiMaterialConfig, usize) =
        decode_from_slice(&bytes).expect("decode multi-material config");
    assert_eq!(val, decoded);
    assert_eq!(decoded.materials.len(), 2);
    assert_eq!(decoded.materials[0].material, FilamentMaterial::Pla);
    assert_eq!(decoded.materials[1].material, FilamentMaterial::Petg);
}

// ---------------------------------------------------------------------------
// Test 11: Post-processing pipeline with multiple steps
// ---------------------------------------------------------------------------

#[test]
fn test_post_processing_pipeline_roundtrip() {
    let val = sample_post_processing();
    let bytes = encode_to_vec(&val).expect("encode post-processing pipeline");
    let (decoded, _): (PostProcessingPipeline, usize) =
        decode_from_slice(&bytes).expect("decode post-processing pipeline");
    assert_eq!(val, decoded);
    assert_eq!(decoded.steps.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 12: Quality control report with dimensional checks and tensile tests
// ---------------------------------------------------------------------------

#[test]
fn test_quality_control_report_roundtrip() {
    let val = sample_quality_report("PART-2026-0042");
    let bytes = encode_to_vec(&val).expect("encode quality report");
    let (decoded, _): (QualityControlReport, usize) =
        decode_from_slice(&bytes).expect("decode quality report");
    assert_eq!(val, decoded);
    assert_eq!(decoded.dimensional_checks.len(), 3);
    assert_eq!(decoded.tensile_tests.len(), 1);
}

// ---------------------------------------------------------------------------
// Test 13: Cost estimation with line items
// ---------------------------------------------------------------------------

#[test]
fn test_cost_estimation_roundtrip() {
    let val = sample_cost_estimation("custom_enclosure");
    let bytes = encode_to_vec(&val).expect("encode cost estimation");
    let (decoded, _): (CostEstimation, usize) =
        decode_from_slice(&bytes).expect("decode cost estimation");
    assert_eq!(val, decoded);
    assert_eq!(decoded.line_items.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 14: Topology optimization result with regions
// ---------------------------------------------------------------------------

#[test]
fn test_topology_optimization_roundtrip() {
    let val = sample_topology_result();
    let bytes = encode_to_vec(&val).expect("encode topology result");
    let (decoded, _): (TopologyOptimizationResult, usize) =
        decode_from_slice(&bytes).expect("decode topology result");
    assert_eq!(val, decoded);
    assert!(!decoded.regions[2].retained);
    assert!(decoded.converged);
}

// ---------------------------------------------------------------------------
// Test 15: Bioprinting scaffold with nested pore geometry
// ---------------------------------------------------------------------------

#[test]
fn test_bioprinting_scaffold_roundtrip() {
    let val = sample_scaffold();
    let bytes = encode_to_vec(&val).expect("encode bioprinting scaffold");
    let (decoded, _): (BioprintingScaffold, usize) =
        decode_from_slice(&bytes).expect("decode bioprinting scaffold");
    assert_eq!(val, decoded);
    assert_eq!(decoded.pore.geometry, "gyroid");
}

// ---------------------------------------------------------------------------
// Test 16: Print farm job (4-level nesting: farm_job -> calibration -> bed_leveling -> probe_points)
// ---------------------------------------------------------------------------

#[test]
fn test_print_farm_job_deep_nesting() {
    let val = sample_print_farm_job("Printer-A1", "phone_case_batch");
    let bytes = encode_to_vec(&val).expect("encode print farm job");
    let (decoded, _): (PrintFarmJob, usize) =
        decode_from_slice(&bytes).expect("decode print farm job");
    assert_eq!(val, decoded);
    assert_eq!(decoded.calibration.bed_leveling.probe_points.len(), 25);
    assert_eq!(decoded.job_config.model.bounding_box.max.z, 60.0);
}

// ---------------------------------------------------------------------------
// Test 17: Print farm status with multiple active jobs
// ---------------------------------------------------------------------------

#[test]
fn test_print_farm_status_roundtrip() {
    let val = PrintFarmStatus {
        farm_name: "Tokyo Manufacturing Center".to_string(),
        location: "Chiyoda-ku, Tokyo".to_string(),
        active_jobs: vec![
            sample_print_farm_job("PR-01", "widget_alpha"),
            sample_print_farm_job("PR-02", "widget_beta"),
            sample_print_farm_job("PR-03", "bracket_gamma"),
        ],
        queued_job_count: 12,
        total_printers: 20,
        online_printers: 17,
    };
    let bytes = encode_to_vec(&val).expect("encode print farm status");
    let (decoded, _): (PrintFarmStatus, usize) =
        decode_from_slice(&bytes).expect("decode print farm status");
    assert_eq!(val, decoded);
    assert_eq!(decoded.active_jobs.len(), 3);
    assert_eq!(decoded.active_jobs[0].printer_id, "PR-01");
}

// ---------------------------------------------------------------------------
// Test 18: Full part report (job + post-processing + quality + cost)
// ---------------------------------------------------------------------------

#[test]
fn test_full_part_report_roundtrip() {
    let val = FullPartReport {
        order_id: "ORD-2026-1337".to_string(),
        job: sample_print_job("turbine_blade"),
        post_processing: sample_post_processing(),
        quality: sample_quality_report("PART-TB-001"),
        cost: sample_cost_estimation("turbine_blade"),
        shipped: false,
    };
    let bytes = encode_to_vec(&val).expect("encode full part report");
    let (decoded, _): (FullPartReport, usize) =
        decode_from_slice(&bytes).expect("decode full part report");
    assert_eq!(val, decoded);
    assert!(!decoded.shipped);
    assert_eq!(decoded.quality.dimensional_checks.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 19: Multi-material print session (calibration + multi-mat + job + gcode)
// ---------------------------------------------------------------------------

#[test]
fn test_multi_material_print_session_roundtrip() {
    let val = MultiMaterialPrintSession {
        session_id: "SESS-20260315-001".to_string(),
        printer_calibration: sample_printer_calibration("Bambu-X1C"),
        multi_material: sample_multi_material_config(),
        job: sample_print_job("dual_color_vase"),
        gcode: sample_gcode_settings(),
    };
    let bytes = encode_to_vec(&val).expect("encode multi-material session");
    let (decoded, _): (MultiMaterialPrintSession, usize) =
        decode_from_slice(&bytes).expect("decode multi-material session");
    assert_eq!(val, decoded);
    assert_eq!(decoded.multi_material.materials.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 20: Bioprinting project (scaffold + topology + quality + cost)
// ---------------------------------------------------------------------------

#[test]
fn test_bioprinting_project_roundtrip() {
    let val = BioprintingProject {
        project_name: "Mandibular Reconstruction Study".to_string(),
        scaffold: sample_scaffold(),
        topology: sample_topology_result(),
        quality: sample_quality_report("BIO-MR-001"),
        cost: sample_cost_estimation("mandibular_scaffold"),
    };
    let bytes = encode_to_vec(&val).expect("encode bioprinting project");
    let (decoded, _): (BioprintingProject, usize) =
        decode_from_slice(&bytes).expect("decode bioprinting project");
    assert_eq!(val, decoded);
    assert_eq!(decoded.scaffold.target_tissue, "trabecular bone");
    assert!(decoded.topology.converged);
}

// ---------------------------------------------------------------------------
// Test 21: Nylon material with four-material config and disabled supports
// ---------------------------------------------------------------------------

#[test]
fn test_four_material_nylon_config() {
    let val = MultiMaterialConfig {
        tool_count: 4,
        materials: vec![
            sample_material(FilamentMaterial::Nylon, "Polymaker", "Natural"),
            sample_material(FilamentMaterial::CarbonFiber, "Priline", "Black"),
            sample_material(FilamentMaterial::Tpu, "NinjaTek", "Midnight"),
            sample_material(FilamentMaterial::Asa, "KVP", "Orange"),
        ],
        purge_tower_enabled: true,
        purge_volume_mm3: 120.0,
        wipe_tower_x: 200.0,
        wipe_tower_y: 180.0,
        tool_change_retraction_mm: 2.0,
    };
    let bytes = encode_to_vec(&val).expect("encode four-material config");
    let (decoded, _): (MultiMaterialConfig, usize) =
        decode_from_slice(&bytes).expect("decode four-material config");
    assert_eq!(val, decoded);
    assert_eq!(decoded.materials.len(), 4);
    assert_eq!(decoded.materials[0].material, FilamentMaterial::Nylon);
    assert!(decoded.materials[0].temperature.chamber_temp_c.is_some());
}

// ---------------------------------------------------------------------------
// Test 22: Filament spool tracking via Vec<MaterialProfile> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_filament_spool_inventory_roundtrip() {
    let val: Vec<MaterialProfile> = vec![
        MaterialProfile {
            material: FilamentMaterial::Pla,
            brand: "eSUN".to_string(),
            color: "Fire Engine Red".to_string(),
            diameter_mm: 1.75,
            density_g_per_cm3: 1.24,
            temperature: sample_temperature_profile(&FilamentMaterial::Pla),
            max_volumetric_speed: 15.0,
            cost_per_kg_usd: 22.0,
            spool_weight_g: 1000,
            remaining_weight_g: 340,
        },
        MaterialProfile {
            material: FilamentMaterial::Petg,
            brand: "Overture".to_string(),
            color: "Transparent Blue".to_string(),
            diameter_mm: 1.75,
            density_g_per_cm3: 1.27,
            temperature: sample_temperature_profile(&FilamentMaterial::Petg),
            max_volumetric_speed: 12.0,
            cost_per_kg_usd: 20.0,
            spool_weight_g: 1000,
            remaining_weight_g: 880,
        },
        MaterialProfile {
            material: FilamentMaterial::Abs,
            brand: "Hatchbox".to_string(),
            color: "True Black".to_string(),
            diameter_mm: 1.75,
            density_g_per_cm3: 1.04,
            temperature: sample_temperature_profile(&FilamentMaterial::Abs),
            max_volumetric_speed: 11.0,
            cost_per_kg_usd: 24.0,
            spool_weight_g: 1000,
            remaining_weight_g: 55,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode spool inventory");
    let (decoded, _): (Vec<MaterialProfile>, usize) =
        decode_from_slice(&bytes).expect("decode spool inventory");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[2].remaining_weight_g, 55);
}
