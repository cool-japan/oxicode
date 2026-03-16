//! Advanced nested structs test #9 — precision machining & CNC manufacturing theme, 22 tests.

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
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InsertShape {
    Round,
    Square,
    Triangle,
    Diamond,
    Trigon,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoatingType {
    TiN,
    TiAlN,
    AlCrN,
    DLC,
    Uncoated,
    CVDAlumina,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WorkholdingType {
    Vice,
    ThreeJawChuck,
    Collet,
    Vacuum,
    MagneticPlate,
    Fixture,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoolantDelivery {
    Flood,
    ThroughSpindle,
    MistCoolant,
    MinimumQuantityLubrication,
    DryMachining,
    CryogenicCO2,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChipEvacuation {
    AirBlast,
    ChipConveyor,
    ScrewAuger,
    Gravity,
    HighPressureCoolant,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MachiningStrategy {
    ThreeAxis,
    ThreePlusTwo,
    FourAxis,
    FiveAxisSimultaneous,
    MillTurn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GCodeInterpolation {
    G00Rapid,
    G01Linear,
    G02ArcCW,
    G03ArcCCW,
    G05HighSpeedMachining,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToleranceType {
    Position,
    Flatness,
    Cylindricity,
    Perpendicularity,
    Concentricity,
    Runout,
    Profile,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenanceCategory {
    Lubrication,
    BallscrewInspection,
    SpindleBearingCheck,
    WayCoversReplacement,
    CoolantChange,
    FilterReplacement,
    GeometricCalibration,
}

// ---------------------------------------------------------------------------
// Domain structs — Level 1 (leaf / shallow)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeGeometry {
    nose_radius_mm: u32,     // stored as microns
    rake_angle_deg_x10: i32, // tenths of degree
    clearance_angle_deg_x10: i32,
    chipbreaker_width_um: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CuttingInsert {
    iso_designation: String,
    shape: InsertShape,
    coating: CoatingType,
    edge: EdgeGeometry,
    max_depth_of_cut_um: u32,
    price_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurfaceFinish {
    ra_nm: u32,
    rz_nm: u32,
    rt_nm: u32,
    measurement_length_um: u32,
    cutoff_wavelength_um: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GdtTolerance {
    tolerance_type: ToleranceType,
    value_um: u32,
    datum_refs: Vec<String>,
    material_condition: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpindleParameters {
    rpm: u32,
    max_rpm: u32,
    power_kw_x10: u32,
    torque_nm_x10: u32,
    bearing_type: String,
    taper_type: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedRate {
    feed_per_tooth_um: u32,
    feed_per_rev_um: u32,
    table_feed_mm_min: u32,
    plunge_rate_mm_min: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialProperties {
    name: String,
    hardness_hrc: u32,
    machinability_index: u32,
    tensile_strength_mpa: u32,
    thermal_conductivity_x10: u32,
    density_kg_m3: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WorkholdingFixture {
    fixture_id: String,
    holding_type: WorkholdingType,
    clamping_force_kn_x10: u32,
    repeatability_um: u32,
    max_workpiece_diameter_mm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoolantSystem {
    delivery: CoolantDelivery,
    pressure_bar_x10: u32,
    flow_rate_lpm_x10: u32,
    concentration_percent_x10: u32,
    tank_capacity_liters: u32,
    filtration_microns: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChipManagement {
    strategy: ChipEvacuation,
    chip_load_um: u32,
    chip_thickness_ratio_x100: u32,
    conveyor_speed_mpm_x10: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToolpathSegment {
    interpolation: GCodeInterpolation,
    start_x_um: i64,
    start_y_um: i64,
    start_z_um: i64,
    end_x_um: i64,
    end_y_um: i64,
    end_z_um: i64,
    arc_radius_um: Option<i64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToolWearReading {
    timestamp_epoch_secs: u64,
    flank_wear_um: u32,
    crater_wear_depth_um: u32,
    notch_wear_um: u32,
    cutting_time_secs: u32,
    parts_machined: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmmProbePoint {
    nominal_x_um: i64,
    nominal_y_um: i64,
    nominal_z_um: i64,
    measured_x_um: i64,
    measured_y_um: i64,
    measured_z_um: i64,
    deviation_um: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceTask {
    task_id: String,
    category: MaintenanceCategory,
    interval_hours: u32,
    last_performed_epoch: u64,
    next_due_epoch: u64,
    estimated_downtime_min: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyReading {
    spindle_kwh_x100: u32,
    axis_drives_kwh_x100: u32,
    coolant_pump_kwh_x100: u32,
    hydraulic_kwh_x100: u32,
    total_kwh_x100: u32,
    parts_produced: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchRecord {
    batch_id: String,
    part_number: String,
    material_heat_number: String,
    quantity_ordered: u32,
    quantity_produced: u32,
    quantity_scrapped: u32,
    start_epoch: u64,
    end_epoch: u64,
}

// ---------------------------------------------------------------------------
// Domain structs — Level 2 (nested)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CuttingTool {
    tool_number: u32,
    description: String,
    insert: CuttingInsert,
    holder_iso: String,
    overhang_mm: u32,
    total_length_mm: u32,
    wear_history: Vec<ToolWearReading>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CncToolpath {
    program_number: String,
    strategy: MachiningStrategy,
    segments: Vec<ToolpathSegment>,
    spindle: SpindleParameters,
    feed: FeedRate,
    coolant: CoolantSystem,
    chip_mgmt: ChipManagement,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmmFeatureResult {
    feature_name: String,
    gdt_spec: GdtTolerance,
    surface_finish: SurfaceFinish,
    probe_points: Vec<CmmProbePoint>,
    in_tolerance: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachineMaintenancePlan {
    machine_id: String,
    machine_model: String,
    tasks: Vec<MaintenanceTask>,
    total_runtime_hours: u64,
    energy: EnergyReading,
}

// ---------------------------------------------------------------------------
// Domain structs — Level 3 (deeply nested)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachiningOperation {
    operation_id: String,
    description: String,
    material: MaterialProperties,
    toolpath: CncToolpath,
    tool: CuttingTool,
    workholding: WorkholdingFixture,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionReport {
    report_id: String,
    inspector: String,
    timestamp_epoch: u64,
    features: Vec<CmmFeatureResult>,
    overall_pass: bool,
}

// ---------------------------------------------------------------------------
// Domain structs — Level 4 (deepest)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionJob {
    job_id: String,
    customer: String,
    operations: Vec<MachiningOperation>,
    inspection: InspectionReport,
    batch: BatchRecord,
    maintenance: MachineMaintenancePlan,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sample_edge_geometry() -> EdgeGeometry {
    EdgeGeometry {
        nose_radius_mm: 800,
        rake_angle_deg_x10: 60,
        clearance_angle_deg_x10: 70,
        chipbreaker_width_um: 1500,
    }
}

fn sample_insert() -> CuttingInsert {
    CuttingInsert {
        iso_designation: "CNMG120408-PM".to_string(),
        shape: InsertShape::Diamond,
        coating: CoatingType::TiAlN,
        edge: sample_edge_geometry(),
        max_depth_of_cut_um: 4000,
        price_cents: 1250,
    }
}

fn sample_surface_finish() -> SurfaceFinish {
    SurfaceFinish {
        ra_nm: 800,
        rz_nm: 4200,
        rt_nm: 5600,
        measurement_length_um: 4000,
        cutoff_wavelength_um: 800,
    }
}

fn sample_gdt(tol_type: ToleranceType, value: u32) -> GdtTolerance {
    GdtTolerance {
        tolerance_type: tol_type,
        value_um: value,
        datum_refs: vec!["A".to_string(), "B".to_string()],
        material_condition: "MMC".to_string(),
    }
}

fn sample_spindle() -> SpindleParameters {
    SpindleParameters {
        rpm: 8000,
        max_rpm: 15000,
        power_kw_x10: 370,
        torque_nm_x10: 2200,
        bearing_type: "Ceramic Hybrid Angular Contact".to_string(),
        taper_type: "HSK-A63".to_string(),
    }
}

fn sample_feed() -> FeedRate {
    FeedRate {
        feed_per_tooth_um: 120,
        feed_per_rev_um: 250,
        table_feed_mm_min: 2400,
        plunge_rate_mm_min: 800,
    }
}

fn sample_material() -> MaterialProperties {
    MaterialProperties {
        name: "Inconel 718".to_string(),
        hardness_hrc: 42,
        machinability_index: 12,
        tensile_strength_mpa: 1240,
        thermal_conductivity_x10: 112,
        density_kg_m3: 8190,
    }
}

fn sample_workholding() -> WorkholdingFixture {
    WorkholdingFixture {
        fixture_id: "FX-2024-007".to_string(),
        holding_type: WorkholdingType::Vice,
        clamping_force_kn_x10: 450,
        repeatability_um: 5,
        max_workpiece_diameter_mm: 300,
    }
}

fn sample_coolant() -> CoolantSystem {
    CoolantSystem {
        delivery: CoolantDelivery::ThroughSpindle,
        pressure_bar_x10: 700,
        flow_rate_lpm_x10: 200,
        concentration_percent_x10: 80,
        tank_capacity_liters: 500,
        filtration_microns: 25,
    }
}

fn sample_chip_mgmt() -> ChipManagement {
    ChipManagement {
        strategy: ChipEvacuation::ChipConveyor,
        chip_load_um: 120,
        chip_thickness_ratio_x100: 85,
        conveyor_speed_mpm_x10: 30,
    }
}

fn sample_segment(interp: GCodeInterpolation) -> ToolpathSegment {
    ToolpathSegment {
        interpolation: interp,
        start_x_um: 0,
        start_y_um: 0,
        start_z_um: 50000,
        end_x_um: 100000,
        end_y_um: 50000,
        end_z_um: -5000,
        arc_radius_um: None,
    }
}

fn sample_wear_reading(time: u64, flank: u32, parts: u32) -> ToolWearReading {
    ToolWearReading {
        timestamp_epoch_secs: time,
        flank_wear_um: flank,
        crater_wear_depth_um: flank / 3,
        notch_wear_um: flank / 5,
        cutting_time_secs: parts * 45,
        parts_machined: parts,
    }
}

fn sample_probe_point(nom_x: i64, nom_y: i64, dev: i32) -> CmmProbePoint {
    CmmProbePoint {
        nominal_x_um: nom_x,
        nominal_y_um: nom_y,
        nominal_z_um: 0,
        measured_x_um: nom_x + dev as i64,
        measured_y_um: nom_y + dev as i64,
        measured_z_um: dev as i64,
        deviation_um: dev,
    }
}

fn sample_cutting_tool() -> CuttingTool {
    CuttingTool {
        tool_number: 1,
        description: "Roughing End Mill 20mm".to_string(),
        insert: sample_insert(),
        holder_iso: "ER32-20".to_string(),
        overhang_mm: 65,
        total_length_mm: 120,
        wear_history: vec![
            sample_wear_reading(1700000000, 50, 10),
            sample_wear_reading(1700003600, 95, 25),
        ],
    }
}

fn sample_toolpath() -> CncToolpath {
    CncToolpath {
        program_number: "O1001".to_string(),
        strategy: MachiningStrategy::ThreePlusTwo,
        segments: vec![
            sample_segment(GCodeInterpolation::G00Rapid),
            sample_segment(GCodeInterpolation::G01Linear),
            sample_segment(GCodeInterpolation::G02ArcCW),
        ],
        spindle: sample_spindle(),
        feed: sample_feed(),
        coolant: sample_coolant(),
        chip_mgmt: sample_chip_mgmt(),
    }
}

fn sample_cmm_feature(name: &str, pass: bool) -> CmmFeatureResult {
    CmmFeatureResult {
        feature_name: name.to_string(),
        gdt_spec: sample_gdt(ToleranceType::Position, 25),
        surface_finish: sample_surface_finish(),
        probe_points: vec![
            sample_probe_point(10000, 20000, 3),
            sample_probe_point(30000, 40000, -2),
        ],
        in_tolerance: pass,
    }
}

fn sample_maintenance_task(id: &str, cat: MaintenanceCategory) -> MaintenanceTask {
    MaintenanceTask {
        task_id: id.to_string(),
        category: cat,
        interval_hours: 500,
        last_performed_epoch: 1700000000,
        next_due_epoch: 1701800000,
        estimated_downtime_min: 120,
    }
}

fn sample_energy() -> EnergyReading {
    EnergyReading {
        spindle_kwh_x100: 1250,
        axis_drives_kwh_x100: 380,
        coolant_pump_kwh_x100: 210,
        hydraulic_kwh_x100: 95,
        total_kwh_x100: 1935,
        parts_produced: 50,
    }
}

fn sample_batch() -> BatchRecord {
    BatchRecord {
        batch_id: "BATCH-2025-0042".to_string(),
        part_number: "PN-718-TURB-003".to_string(),
        material_heat_number: "HN-2024-11-0089".to_string(),
        quantity_ordered: 100,
        quantity_produced: 98,
        quantity_scrapped: 2,
        start_epoch: 1700000000,
        end_epoch: 1700172800,
    }
}

fn sample_inspection() -> InspectionReport {
    InspectionReport {
        report_id: "IR-2025-0137".to_string(),
        inspector: "Takeshi Yamamoto".to_string(),
        timestamp_epoch: 1700086400,
        features: vec![
            sample_cmm_feature("Bore Diameter 45H7", true),
            sample_cmm_feature("Face Flatness", true),
        ],
        overall_pass: true,
    }
}

fn sample_machining_op(op_id: &str) -> MachiningOperation {
    MachiningOperation {
        operation_id: op_id.to_string(),
        description: "Rough turning of turbine disc".to_string(),
        material: sample_material(),
        toolpath: sample_toolpath(),
        tool: sample_cutting_tool(),
        workholding: sample_workholding(),
    }
}

fn sample_maintenance_plan() -> MachineMaintenancePlan {
    MachineMaintenancePlan {
        machine_id: "DMU-65-monoBLOCK-007".to_string(),
        machine_model: "DMG MORI DMU 65 monoBLOCK".to_string(),
        tasks: vec![
            sample_maintenance_task("MT-001", MaintenanceCategory::Lubrication),
            sample_maintenance_task("MT-002", MaintenanceCategory::SpindleBearingCheck),
        ],
        total_runtime_hours: 12450,
        energy: sample_energy(),
    }
}

fn sample_production_job() -> ProductionJob {
    ProductionJob {
        job_id: "JOB-2025-0019".to_string(),
        customer: "Aero Dynamics Corp".to_string(),
        operations: vec![sample_machining_op("OP-10"), sample_machining_op("OP-20")],
        inspection: sample_inspection(),
        batch: sample_batch(),
        maintenance: sample_maintenance_plan(),
    }
}

// ---------------------------------------------------------------------------
// Test 1: Edge geometry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_edge_geometry_roundtrip() {
    let val = sample_edge_geometry();
    let bytes = encode_to_vec(&val).expect("encode edge geometry");
    let (decoded, _): (EdgeGeometry, usize) =
        decode_from_slice(&bytes).expect("decode edge geometry");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Cutting insert with coating and shape
// ---------------------------------------------------------------------------

#[test]
fn test_cutting_insert_roundtrip() {
    let val = sample_insert();
    let bytes = encode_to_vec(&val).expect("encode cutting insert");
    let (decoded, _): (CuttingInsert, usize) =
        decode_from_slice(&bytes).expect("decode cutting insert");
    assert_eq!(val, decoded);
    assert_eq!(decoded.shape, InsertShape::Diamond);
    assert_eq!(decoded.coating, CoatingType::TiAlN);
}

// ---------------------------------------------------------------------------
// Test 3: Surface finish measurements (Ra, Rz, Rt)
// ---------------------------------------------------------------------------

#[test]
fn test_surface_finish_roundtrip() {
    let val = sample_surface_finish();
    let bytes = encode_to_vec(&val).expect("encode surface finish");
    let (decoded, _): (SurfaceFinish, usize) =
        decode_from_slice(&bytes).expect("decode surface finish");
    assert_eq!(val, decoded);
    assert!(decoded.ra_nm < decoded.rz_nm);
    assert!(decoded.rz_nm < decoded.rt_nm);
}

// ---------------------------------------------------------------------------
// Test 4: GD&T tolerance spec with datum references
// ---------------------------------------------------------------------------

#[test]
fn test_gdt_tolerance_roundtrip() {
    let val = sample_gdt(ToleranceType::Cylindricity, 10);
    let bytes = encode_to_vec(&val).expect("encode gdt");
    let (decoded, _): (GdtTolerance, usize) = decode_from_slice(&bytes).expect("decode gdt");
    assert_eq!(val, decoded);
    assert_eq!(decoded.datum_refs.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 5: Spindle speed and torque parameters
// ---------------------------------------------------------------------------

#[test]
fn test_spindle_parameters_roundtrip() {
    let val = sample_spindle();
    let bytes = encode_to_vec(&val).expect("encode spindle");
    let (decoded, _): (SpindleParameters, usize) =
        decode_from_slice(&bytes).expect("decode spindle");
    assert_eq!(val, decoded);
    assert!(decoded.rpm <= decoded.max_rpm);
}

// ---------------------------------------------------------------------------
// Test 6: Feed rate optimization parameters
// ---------------------------------------------------------------------------

#[test]
fn test_feed_rate_roundtrip() {
    let val = sample_feed();
    let bytes = encode_to_vec(&val).expect("encode feed rate");
    let (decoded, _): (FeedRate, usize) = decode_from_slice(&bytes).expect("decode feed rate");
    assert_eq!(val, decoded);
    assert!(decoded.plunge_rate_mm_min < decoded.table_feed_mm_min);
}

// ---------------------------------------------------------------------------
// Test 7: Material properties (Inconel 718 superalloy)
// ---------------------------------------------------------------------------

#[test]
fn test_material_properties_roundtrip() {
    let val = sample_material();
    let bytes = encode_to_vec(&val).expect("encode material");
    let (decoded, _): (MaterialProperties, usize) =
        decode_from_slice(&bytes).expect("decode material");
    assert_eq!(val, decoded);
    assert_eq!(decoded.name, "Inconel 718");
}

// ---------------------------------------------------------------------------
// Test 8: Workholding fixture (vice)
// ---------------------------------------------------------------------------

#[test]
fn test_workholding_fixture_roundtrip() {
    let val = sample_workholding();
    let bytes = encode_to_vec(&val).expect("encode workholding");
    let (decoded, _): (WorkholdingFixture, usize) =
        decode_from_slice(&bytes).expect("decode workholding");
    assert_eq!(val, decoded);
    assert_eq!(decoded.holding_type, WorkholdingType::Vice);
}

// ---------------------------------------------------------------------------
// Test 9: Coolant delivery system (through-spindle high pressure)
// ---------------------------------------------------------------------------

#[test]
fn test_coolant_system_roundtrip() {
    let val = sample_coolant();
    let bytes = encode_to_vec(&val).expect("encode coolant");
    let (decoded, _): (CoolantSystem, usize) = decode_from_slice(&bytes).expect("decode coolant");
    assert_eq!(val, decoded);
    assert_eq!(decoded.delivery, CoolantDelivery::ThroughSpindle);
}

// ---------------------------------------------------------------------------
// Test 10: Chip evacuation strategy
// ---------------------------------------------------------------------------

#[test]
fn test_chip_management_roundtrip() {
    let val = sample_chip_mgmt();
    let bytes = encode_to_vec(&val).expect("encode chip mgmt");
    let (decoded, _): (ChipManagement, usize) =
        decode_from_slice(&bytes).expect("decode chip mgmt");
    assert_eq!(val, decoded);
    assert_eq!(decoded.strategy, ChipEvacuation::ChipConveyor);
}

// ---------------------------------------------------------------------------
// Test 11: Toolpath segment with arc interpolation
// ---------------------------------------------------------------------------

#[test]
fn test_toolpath_segment_arc_roundtrip() {
    let val = ToolpathSegment {
        interpolation: GCodeInterpolation::G02ArcCW,
        start_x_um: 50000,
        start_y_um: 0,
        start_z_um: -3000,
        end_x_um: 0,
        end_y_um: 50000,
        end_z_um: -3000,
        arc_radius_um: Some(50000),
    };
    let bytes = encode_to_vec(&val).expect("encode arc segment");
    let (decoded, _): (ToolpathSegment, usize) =
        decode_from_slice(&bytes).expect("decode arc segment");
    assert_eq!(val, decoded);
    assert_eq!(decoded.arc_radius_um, Some(50000));
}

// ---------------------------------------------------------------------------
// Test 12: Tool wear monitoring over time
// ---------------------------------------------------------------------------

#[test]
fn test_tool_wear_history_roundtrip() {
    let val = CuttingTool {
        tool_number: 5,
        description: "Finish Ball Nose 10mm".to_string(),
        insert: CuttingInsert {
            iso_designation: "RDHX1003MOT-MD".to_string(),
            shape: InsertShape::Round,
            coating: CoatingType::AlCrN,
            edge: EdgeGeometry {
                nose_radius_mm: 5000,
                rake_angle_deg_x10: 120,
                clearance_angle_deg_x10: 110,
                chipbreaker_width_um: 0,
            },
            max_depth_of_cut_um: 1500,
            price_cents: 2890,
        },
        holder_iso: "HSK-A63-ER25".to_string(),
        overhang_mm: 45,
        total_length_mm: 95,
        wear_history: vec![
            sample_wear_reading(1700000000, 20, 5),
            sample_wear_reading(1700007200, 55, 15),
            sample_wear_reading(1700014400, 110, 30),
            sample_wear_reading(1700021600, 180, 42),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode cutting tool with wear");
    let (decoded, _): (CuttingTool, usize) =
        decode_from_slice(&bytes).expect("decode cutting tool with wear");
    assert_eq!(val, decoded);
    assert_eq!(decoded.wear_history.len(), 4);
    // Verify progressive wear
    for pair in decoded.wear_history.windows(2) {
        assert!(pair[0].flank_wear_um < pair[1].flank_wear_um);
    }
}

// ---------------------------------------------------------------------------
// Test 13: CMM inspection feature result (2 levels deep)
// ---------------------------------------------------------------------------

#[test]
fn test_cmm_feature_result_roundtrip() {
    let val = CmmFeatureResult {
        feature_name: "Bore 45H7 Position".to_string(),
        gdt_spec: GdtTolerance {
            tolerance_type: ToleranceType::Position,
            value_um: 25,
            datum_refs: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            material_condition: "MMC".to_string(),
        },
        surface_finish: SurfaceFinish {
            ra_nm: 400,
            rz_nm: 2100,
            rt_nm: 2800,
            measurement_length_um: 4000,
            cutoff_wavelength_um: 800,
        },
        probe_points: vec![
            sample_probe_point(0, 0, 2),
            sample_probe_point(22500, 0, -1),
            sample_probe_point(0, 22500, 3),
            sample_probe_point(-22500, 0, -2),
            sample_probe_point(0, -22500, 1),
        ],
        in_tolerance: true,
    };
    let bytes = encode_to_vec(&val).expect("encode cmm feature");
    let (decoded, _): (CmmFeatureResult, usize) =
        decode_from_slice(&bytes).expect("decode cmm feature");
    assert_eq!(val, decoded);
    assert_eq!(decoded.probe_points.len(), 5);
    assert!(decoded.in_tolerance);
}

// ---------------------------------------------------------------------------
// Test 14: CNC toolpath with multi-axis strategy (3 levels deep)
// ---------------------------------------------------------------------------

#[test]
fn test_cnc_toolpath_five_axis_roundtrip() {
    let val = CncToolpath {
        program_number: "O5001".to_string(),
        strategy: MachiningStrategy::FiveAxisSimultaneous,
        segments: vec![
            sample_segment(GCodeInterpolation::G00Rapid),
            sample_segment(GCodeInterpolation::G01Linear),
            ToolpathSegment {
                interpolation: GCodeInterpolation::G05HighSpeedMachining,
                start_x_um: 100000,
                start_y_um: 50000,
                start_z_um: -5000,
                end_x_um: 200000,
                end_y_um: 75000,
                end_z_um: -8000,
                arc_radius_um: None,
            },
        ],
        spindle: SpindleParameters {
            rpm: 12000,
            max_rpm: 18000,
            power_kw_x10: 520,
            torque_nm_x10: 1800,
            bearing_type: "Full Ceramic".to_string(),
            taper_type: "HSK-E50".to_string(),
        },
        feed: FeedRate {
            feed_per_tooth_um: 80,
            feed_per_rev_um: 160,
            table_feed_mm_min: 3600,
            plunge_rate_mm_min: 600,
        },
        coolant: CoolantSystem {
            delivery: CoolantDelivery::CryogenicCO2,
            pressure_bar_x10: 1500,
            flow_rate_lpm_x10: 50,
            concentration_percent_x10: 0,
            tank_capacity_liters: 80,
            filtration_microns: 0,
        },
        chip_mgmt: ChipManagement {
            strategy: ChipEvacuation::AirBlast,
            chip_load_um: 80,
            chip_thickness_ratio_x100: 72,
            conveyor_speed_mpm_x10: 0,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode 5-axis toolpath");
    let (decoded, _): (CncToolpath, usize) =
        decode_from_slice(&bytes).expect("decode 5-axis toolpath");
    assert_eq!(val, decoded);
    assert_eq!(decoded.strategy, MachiningStrategy::FiveAxisSimultaneous);
    assert_eq!(decoded.segments.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 15: Full machining operation (3 levels deep)
// ---------------------------------------------------------------------------

#[test]
fn test_machining_operation_roundtrip() {
    let val = sample_machining_op("OP-10");
    let bytes = encode_to_vec(&val).expect("encode machining op");
    let (decoded, _): (MachiningOperation, usize) =
        decode_from_slice(&bytes).expect("decode machining op");
    assert_eq!(val, decoded);
    assert_eq!(decoded.tool.insert.shape, InsertShape::Diamond);
    assert_eq!(decoded.toolpath.strategy, MachiningStrategy::ThreePlusTwo);
}

// ---------------------------------------------------------------------------
// Test 16: Inspection report with multiple features
// ---------------------------------------------------------------------------

#[test]
fn test_inspection_report_roundtrip() {
    let val = InspectionReport {
        report_id: "IR-2025-0200".to_string(),
        inspector: "Kenji Watanabe".to_string(),
        timestamp_epoch: 1700100000,
        features: vec![
            sample_cmm_feature("Bore 45H7", true),
            sample_cmm_feature("Face Runout", true),
            CmmFeatureResult {
                feature_name: "Slot Width 12js9".to_string(),
                gdt_spec: sample_gdt(ToleranceType::Profile, 15),
                surface_finish: SurfaceFinish {
                    ra_nm: 1600,
                    rz_nm: 8000,
                    rt_nm: 11000,
                    measurement_length_um: 4000,
                    cutoff_wavelength_um: 800,
                },
                probe_points: vec![sample_probe_point(6000, 0, -8)],
                in_tolerance: false,
            },
        ],
        overall_pass: false,
    };
    let bytes = encode_to_vec(&val).expect("encode inspection report");
    let (decoded, _): (InspectionReport, usize) =
        decode_from_slice(&bytes).expect("decode inspection report");
    assert_eq!(val, decoded);
    assert!(!decoded.overall_pass);
    assert_eq!(decoded.features.len(), 3);
    assert!(!decoded.features[2].in_tolerance);
}

// ---------------------------------------------------------------------------
// Test 17: Machine maintenance plan with energy tracking
// ---------------------------------------------------------------------------

#[test]
fn test_maintenance_plan_roundtrip() {
    let val = MachineMaintenancePlan {
        machine_id: "MAK-NJX-5500-012".to_string(),
        machine_model: "Nakamura-Tome NJX-5500".to_string(),
        tasks: vec![
            sample_maintenance_task("MT-L01", MaintenanceCategory::Lubrication),
            sample_maintenance_task("MT-B01", MaintenanceCategory::BallscrewInspection),
            sample_maintenance_task("MT-S01", MaintenanceCategory::SpindleBearingCheck),
            sample_maintenance_task("MT-C01", MaintenanceCategory::CoolantChange),
            sample_maintenance_task("MT-F01", MaintenanceCategory::FilterReplacement),
            sample_maintenance_task("MT-G01", MaintenanceCategory::GeometricCalibration),
        ],
        total_runtime_hours: 25800,
        energy: EnergyReading {
            spindle_kwh_x100: 2500,
            axis_drives_kwh_x100: 780,
            coolant_pump_kwh_x100: 420,
            hydraulic_kwh_x100: 190,
            total_kwh_x100: 3890,
            parts_produced: 120,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode maintenance plan");
    let (decoded, _): (MachineMaintenancePlan, usize) =
        decode_from_slice(&bytes).expect("decode maintenance plan");
    assert_eq!(val, decoded);
    assert_eq!(decoded.tasks.len(), 6);
    // Verify energy per part
    let kwh_per_part = decoded.energy.total_kwh_x100 / decoded.energy.parts_produced;
    assert!(kwh_per_part > 0);
}

// ---------------------------------------------------------------------------
// Test 18: Production batch traceability
// ---------------------------------------------------------------------------

#[test]
fn test_batch_record_roundtrip() {
    let val = BatchRecord {
        batch_id: "BATCH-2025-0100".to_string(),
        part_number: "PN-TI64-IMPLANT-007".to_string(),
        material_heat_number: "HN-TI64-2025-01-0034".to_string(),
        quantity_ordered: 500,
        quantity_produced: 497,
        quantity_scrapped: 3,
        start_epoch: 1700000000,
        end_epoch: 1700345600,
    };
    let bytes = encode_to_vec(&val).expect("encode batch record");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&bytes).expect("decode batch record");
    assert_eq!(val, decoded);
    assert_eq!(decoded.quantity_produced + decoded.quantity_scrapped, 500);
}

// ---------------------------------------------------------------------------
// Test 19: Full production job (4 levels deep)
// ---------------------------------------------------------------------------

#[test]
fn test_production_job_full_roundtrip() {
    let val = sample_production_job();
    let bytes = encode_to_vec(&val).expect("encode production job");
    let (decoded, _): (ProductionJob, usize) =
        decode_from_slice(&bytes).expect("decode production job");
    assert_eq!(val, decoded);
    assert_eq!(decoded.operations.len(), 2);
    assert!(decoded.inspection.overall_pass);
    // Verify deep nesting access (4 levels)
    assert_eq!(decoded.operations[0].tool.insert.edge.nose_radius_mm, 800);
}

// ---------------------------------------------------------------------------
// Test 20: Multiple insert shapes and coatings
// ---------------------------------------------------------------------------

#[test]
fn test_various_insert_shapes_coatings() {
    let inserts = vec![
        CuttingInsert {
            iso_designation: "RCMT1204MO".to_string(),
            shape: InsertShape::Round,
            coating: CoatingType::TiN,
            edge: EdgeGeometry {
                nose_radius_mm: 6000,
                rake_angle_deg_x10: 0,
                clearance_angle_deg_x10: 70,
                chipbreaker_width_um: 0,
            },
            max_depth_of_cut_um: 3000,
            price_cents: 980,
        },
        CuttingInsert {
            iso_designation: "SNMG120408".to_string(),
            shape: InsertShape::Square,
            coating: CoatingType::CVDAlumina,
            edge: EdgeGeometry {
                nose_radius_mm: 800,
                rake_angle_deg_x10: -60,
                clearance_angle_deg_x10: 50,
                chipbreaker_width_um: 2000,
            },
            max_depth_of_cut_um: 6000,
            price_cents: 750,
        },
        CuttingInsert {
            iso_designation: "TNMG160408".to_string(),
            shape: InsertShape::Triangle,
            coating: CoatingType::DLC,
            edge: EdgeGeometry {
                nose_radius_mm: 800,
                rake_angle_deg_x10: 50,
                clearance_angle_deg_x10: 60,
                chipbreaker_width_um: 1800,
            },
            max_depth_of_cut_um: 4500,
            price_cents: 1100,
        },
        CuttingInsert {
            iso_designation: "TCMT110204-PF".to_string(),
            shape: InsertShape::Trigon,
            coating: CoatingType::Uncoated,
            edge: EdgeGeometry {
                nose_radius_mm: 400,
                rake_angle_deg_x10: 70,
                clearance_angle_deg_x10: 70,
                chipbreaker_width_um: 1200,
            },
            max_depth_of_cut_um: 2000,
            price_cents: 540,
        },
    ];
    let bytes = encode_to_vec(&inserts).expect("encode inserts vec");
    let (decoded, _): (Vec<CuttingInsert>, usize) =
        decode_from_slice(&bytes).expect("decode inserts vec");
    assert_eq!(inserts, decoded);
    assert_eq!(decoded.len(), 4);
}

// ---------------------------------------------------------------------------
// Test 21: Multiple workholding types in fixture library
// ---------------------------------------------------------------------------

#[test]
fn test_workholding_types_roundtrip() {
    let fixtures = vec![
        WorkholdingFixture {
            fixture_id: "FX-CHUCK-001".to_string(),
            holding_type: WorkholdingType::ThreeJawChuck,
            clamping_force_kn_x10: 600,
            repeatability_um: 15,
            max_workpiece_diameter_mm: 250,
        },
        WorkholdingFixture {
            fixture_id: "FX-COLLET-005".to_string(),
            holding_type: WorkholdingType::Collet,
            clamping_force_kn_x10: 200,
            repeatability_um: 3,
            max_workpiece_diameter_mm: 32,
        },
        WorkholdingFixture {
            fixture_id: "FX-VAC-012".to_string(),
            holding_type: WorkholdingType::Vacuum,
            clamping_force_kn_x10: 80,
            repeatability_um: 10,
            max_workpiece_diameter_mm: 500,
        },
        WorkholdingFixture {
            fixture_id: "FX-MAG-003".to_string(),
            holding_type: WorkholdingType::MagneticPlate,
            clamping_force_kn_x10: 350,
            repeatability_um: 8,
            max_workpiece_diameter_mm: 400,
        },
        WorkholdingFixture {
            fixture_id: "FX-CUSTOM-099".to_string(),
            holding_type: WorkholdingType::Fixture,
            clamping_force_kn_x10: 500,
            repeatability_um: 2,
            max_workpiece_diameter_mm: 150,
        },
    ];
    let bytes = encode_to_vec(&fixtures).expect("encode fixtures");
    let (decoded, _): (Vec<WorkholdingFixture>, usize) =
        decode_from_slice(&bytes).expect("decode fixtures");
    assert_eq!(fixtures, decoded);
    assert_eq!(decoded.len(), 5);
}

// ---------------------------------------------------------------------------
// Test 22: Multi-operation job with different materials and strategies
// ---------------------------------------------------------------------------

#[test]
fn test_multi_operation_production_job() {
    let val = ProductionJob {
        job_id: "JOB-2025-AERO-0055".to_string(),
        customer: "Precision Aerospace Ltd".to_string(),
        operations: vec![
            MachiningOperation {
                operation_id: "OP-10-ROUGH".to_string(),
                description: "Rough milling of blisk profile".to_string(),
                material: MaterialProperties {
                    name: "Ti-6Al-4V".to_string(),
                    hardness_hrc: 36,
                    machinability_index: 22,
                    tensile_strength_mpa: 950,
                    thermal_conductivity_x10: 67,
                    density_kg_m3: 4430,
                },
                toolpath: CncToolpath {
                    program_number: "O8010".to_string(),
                    strategy: MachiningStrategy::FiveAxisSimultaneous,
                    segments: vec![
                        sample_segment(GCodeInterpolation::G00Rapid),
                        sample_segment(GCodeInterpolation::G01Linear),
                    ],
                    spindle: SpindleParameters {
                        rpm: 6000,
                        max_rpm: 20000,
                        power_kw_x10: 600,
                        torque_nm_x10: 3000,
                        bearing_type: "Ceramic Hybrid".to_string(),
                        taper_type: "HSK-A100".to_string(),
                    },
                    feed: FeedRate {
                        feed_per_tooth_um: 150,
                        feed_per_rev_um: 300,
                        table_feed_mm_min: 1800,
                        plunge_rate_mm_min: 500,
                    },
                    coolant: CoolantSystem {
                        delivery: CoolantDelivery::ThroughSpindle,
                        pressure_bar_x10: 1000,
                        flow_rate_lpm_x10: 300,
                        concentration_percent_x10: 90,
                        tank_capacity_liters: 800,
                        filtration_microns: 10,
                    },
                    chip_mgmt: ChipManagement {
                        strategy: ChipEvacuation::HighPressureCoolant,
                        chip_load_um: 150,
                        chip_thickness_ratio_x100: 90,
                        conveyor_speed_mpm_x10: 40,
                    },
                },
                tool: CuttingTool {
                    tool_number: 1,
                    description: "Roughing barrel cutter 25mm".to_string(),
                    insert: CuttingInsert {
                        iso_designation: "CUSTOM-BARREL-25".to_string(),
                        shape: InsertShape::Round,
                        coating: CoatingType::AlCrN,
                        edge: EdgeGeometry {
                            nose_radius_mm: 12500,
                            rake_angle_deg_x10: 80,
                            clearance_angle_deg_x10: 100,
                            chipbreaker_width_um: 2500,
                        },
                        max_depth_of_cut_um: 8000,
                        price_cents: 8900,
                    },
                    holder_iso: "HSK-A100-SHRINK-25".to_string(),
                    overhang_mm: 80,
                    total_length_mm: 150,
                    wear_history: vec![
                        sample_wear_reading(1700000000, 30, 8),
                        sample_wear_reading(1700010800, 75, 20),
                    ],
                },
                workholding: WorkholdingFixture {
                    fixture_id: "FX-BLISK-001".to_string(),
                    holding_type: WorkholdingType::Fixture,
                    clamping_force_kn_x10: 800,
                    repeatability_um: 3,
                    max_workpiece_diameter_mm: 600,
                },
            },
            MachiningOperation {
                operation_id: "OP-20-FINISH".to_string(),
                description: "Finish polishing of blade surfaces".to_string(),
                material: MaterialProperties {
                    name: "Ti-6Al-4V".to_string(),
                    hardness_hrc: 36,
                    machinability_index: 22,
                    tensile_strength_mpa: 950,
                    thermal_conductivity_x10: 67,
                    density_kg_m3: 4430,
                },
                toolpath: CncToolpath {
                    program_number: "O8020".to_string(),
                    strategy: MachiningStrategy::FiveAxisSimultaneous,
                    segments: vec![
                        sample_segment(GCodeInterpolation::G00Rapid),
                        sample_segment(GCodeInterpolation::G05HighSpeedMachining),
                    ],
                    spindle: SpindleParameters {
                        rpm: 18000,
                        max_rpm: 20000,
                        power_kw_x10: 200,
                        torque_nm_x10: 800,
                        bearing_type: "Ceramic Hybrid".to_string(),
                        taper_type: "HSK-E50".to_string(),
                    },
                    feed: FeedRate {
                        feed_per_tooth_um: 40,
                        feed_per_rev_um: 80,
                        table_feed_mm_min: 5400,
                        plunge_rate_mm_min: 200,
                    },
                    coolant: CoolantSystem {
                        delivery: CoolantDelivery::MinimumQuantityLubrication,
                        pressure_bar_x10: 60,
                        flow_rate_lpm_x10: 5,
                        concentration_percent_x10: 1000,
                        tank_capacity_liters: 20,
                        filtration_microns: 0,
                    },
                    chip_mgmt: ChipManagement {
                        strategy: ChipEvacuation::AirBlast,
                        chip_load_um: 40,
                        chip_thickness_ratio_x100: 60,
                        conveyor_speed_mpm_x10: 0,
                    },
                },
                tool: CuttingTool {
                    tool_number: 12,
                    description: "Ball nose finish 6mm PCD".to_string(),
                    insert: CuttingInsert {
                        iso_designation: "PCD-BALL-6R3".to_string(),
                        shape: InsertShape::Round,
                        coating: CoatingType::Uncoated,
                        edge: EdgeGeometry {
                            nose_radius_mm: 3000,
                            rake_angle_deg_x10: 50,
                            clearance_angle_deg_x10: 120,
                            chipbreaker_width_um: 0,
                        },
                        max_depth_of_cut_um: 500,
                        price_cents: 24500,
                    },
                    holder_iso: "HSK-E50-SHRINK-6".to_string(),
                    overhang_mm: 35,
                    total_length_mm: 75,
                    wear_history: vec![sample_wear_reading(1700014400, 5, 12)],
                },
                workholding: WorkholdingFixture {
                    fixture_id: "FX-BLISK-001".to_string(),
                    holding_type: WorkholdingType::Fixture,
                    clamping_force_kn_x10: 800,
                    repeatability_um: 3,
                    max_workpiece_diameter_mm: 600,
                },
            },
        ],
        inspection: InspectionReport {
            report_id: "IR-BLISK-2025-001".to_string(),
            inspector: "Dr. Yuki Tanaka".to_string(),
            timestamp_epoch: 1700100000,
            features: vec![
                CmmFeatureResult {
                    feature_name: "Blade Profile Zone A".to_string(),
                    gdt_spec: GdtTolerance {
                        tolerance_type: ToleranceType::Profile,
                        value_um: 15,
                        datum_refs: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                        material_condition: "RFS".to_string(),
                    },
                    surface_finish: SurfaceFinish {
                        ra_nm: 200,
                        rz_nm: 1100,
                        rt_nm: 1400,
                        measurement_length_um: 4000,
                        cutoff_wavelength_um: 250,
                    },
                    probe_points: vec![
                        sample_probe_point(0, 0, 1),
                        sample_probe_point(5000, 10000, -1),
                        sample_probe_point(10000, 20000, 2),
                    ],
                    in_tolerance: true,
                },
                CmmFeatureResult {
                    feature_name: "Root Fillet Radius".to_string(),
                    gdt_spec: sample_gdt(ToleranceType::Runout, 20),
                    surface_finish: SurfaceFinish {
                        ra_nm: 400,
                        rz_nm: 2200,
                        rt_nm: 2900,
                        measurement_length_um: 2000,
                        cutoff_wavelength_um: 250,
                    },
                    probe_points: vec![
                        sample_probe_point(0, 0, -3),
                        sample_probe_point(1000, 0, 2),
                    ],
                    in_tolerance: true,
                },
            ],
            overall_pass: true,
        },
        batch: BatchRecord {
            batch_id: "BATCH-BLISK-2025-003".to_string(),
            part_number: "PN-BLISK-FAN-STG1".to_string(),
            material_heat_number: "HN-TI64-FORGING-2025-0012".to_string(),
            quantity_ordered: 24,
            quantity_produced: 23,
            quantity_scrapped: 1,
            start_epoch: 1700000000,
            end_epoch: 1700604800,
        },
        maintenance: MachineMaintenancePlan {
            machine_id: "HERM-C52U-MT-003".to_string(),
            machine_model: "Hermle C 52 U MT".to_string(),
            tasks: vec![
                sample_maintenance_task("MT-LUB-W", MaintenanceCategory::Lubrication),
                sample_maintenance_task("MT-GEO-Q", MaintenanceCategory::GeometricCalibration),
                sample_maintenance_task("MT-WAY-Y", MaintenanceCategory::WayCoversReplacement),
            ],
            total_runtime_hours: 8750,
            energy: EnergyReading {
                spindle_kwh_x100: 4200,
                axis_drives_kwh_x100: 1500,
                coolant_pump_kwh_x100: 800,
                hydraulic_kwh_x100: 350,
                total_kwh_x100: 6850,
                parts_produced: 23,
            },
        },
    };
    let bytes = encode_to_vec(&val).expect("encode multi-op production job");
    let (decoded, _): (ProductionJob, usize) =
        decode_from_slice(&bytes).expect("decode multi-op production job");
    assert_eq!(val, decoded);
    // Verify deep nesting (4 levels): job -> operation -> tool -> insert -> edge
    assert_eq!(decoded.operations[0].tool.insert.edge.nose_radius_mm, 12500);
    assert_eq!(
        decoded.operations[1].tool.insert.coating,
        CoatingType::Uncoated
    );
    assert_eq!(decoded.inspection.features.len(), 2);
    assert!(decoded.inspection.overall_pass);
    assert_eq!(decoded.maintenance.tasks.len(), 3);
    // Energy per part for blisk production
    let energy_per_part =
        decoded.maintenance.energy.total_kwh_x100 / decoded.maintenance.energy.parts_produced;
    assert!(energy_per_part > 200);
}
