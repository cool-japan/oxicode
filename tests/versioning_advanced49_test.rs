#![cfg(feature = "versioning")]

//! Versioning tests for OxiCode: Construction Project Management & BIM domain.
//!
//! 22 test functions covering structural elements, project scheduling, material
//! takeoffs, inspection checklists, change orders, RFI logs, concrete mix
//! designs, rebar schedules, MEP clash detection, safety incidents, and
//! subcontractor progress claims.

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
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ── Domain enums ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StructuralElementKind {
    Beam,
    Column,
    Slab,
    Wall,
    Foundation,
    Truss,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaterialGrade {
    ConcreteC25,
    ConcreteC30,
    ConcreteC40,
    ConcreteC50,
    SteelS235,
    SteelS355,
    SteelS460,
    TimberC24,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MilestoneStatus {
    NotStarted,
    InProgress,
    Completed,
    Delayed,
    OnHold,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InspectionResult {
    Pass,
    ConditionalPass,
    Fail,
    Reinspect,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChangeOrderStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Implemented,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RfiPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RfiStatus {
    Open,
    Answered,
    Closed,
    Overdue,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClashSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MepDiscipline {
    Mechanical,
    Electrical,
    Plumbing,
    FireProtection,
    Hvac,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentSeverity {
    NearMiss,
    FirstAid,
    MedicalTreatment,
    LostTime,
    Fatality,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClaimStatus {
    Draft,
    Submitted,
    Certified,
    Disputed,
    Paid,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RebarGrade {
    Grade40,
    Grade60,
    Grade75,
    Grade80,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConcretePlacement {
    PumpedInPlace,
    CraneBucket,
    DirectChute,
    ShotcreteWet,
    ShotcreteDry,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Clear,
    Overcast,
    LightRain,
    HeavyRain,
    Snow,
    ExtremeHeat,
    HighWind,
}

// ── Domain structs ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StructuralElement {
    element_id: u64,
    kind: StructuralElementKind,
    grade: MaterialGrade,
    label: String,
    length_mm: u32,
    width_mm: u32,
    depth_mm: u32,
    floor_level: i16,
    load_capacity_kn: u32,
    is_critical_path: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProjectMilestone {
    milestone_id: u64,
    name: String,
    planned_start_day: u32,
    planned_end_day: u32,
    actual_start_day: u32,
    actual_end_day: u32,
    status: MilestoneStatus,
    percent_complete: u8,
    predecessor_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialTakeoffItem {
    item_id: u64,
    description: String,
    unit: String,
    quantity: u64,
    unit_cost_cents: u64,
    waste_factor_pct: u8,
    supplier: String,
    lead_time_days: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionChecklistEntry {
    entry_id: u64,
    description: String,
    result: InspectionResult,
    inspector_name: String,
    comments: String,
    photo_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiteInspection {
    inspection_id: u64,
    project_name: String,
    date_epoch: u64,
    inspector: String,
    weather: WeatherCondition,
    entries: Vec<InspectionChecklistEntry>,
    overall_result: InspectionResult,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChangeOrder {
    co_number: u32,
    title: String,
    description: String,
    status: ChangeOrderStatus,
    requested_by: String,
    cost_impact_cents: i64,
    schedule_impact_days: i32,
    affected_elements: Vec<u64>,
    date_submitted_epoch: u64,
    date_resolved_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RfiRecord {
    rfi_number: u32,
    subject: String,
    question: String,
    answer: String,
    priority: RfiPriority,
    status: RfiStatus,
    submitted_by: String,
    assigned_to: String,
    date_opened_epoch: u64,
    date_closed_epoch: u64,
    related_drawing_refs: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConcreteMixDesign {
    mix_id: u64,
    designation: String,
    target_strength_mpa: u16,
    water_cement_ratio: u16, // stored as ratio * 1000
    cement_kg_per_m3: u16,
    water_kg_per_m3: u16,
    fine_aggregate_kg: u16,
    coarse_aggregate_kg: u16,
    admixture_ml_per_m3: u32,
    slump_mm: u16,
    max_aggregate_size_mm: u8,
    placement_method: ConcretePlacement,
    air_content_pct_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RebarScheduleEntry {
    bar_mark: String,
    grade: RebarGrade,
    diameter_mm: u8,
    length_mm: u32,
    quantity: u32,
    shape_code: String,
    bending_dimension_a_mm: u32,
    bending_dimension_b_mm: u32,
    element_ref: String,
    total_weight_grams: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MepClashResult {
    clash_id: u64,
    discipline_a: MepDiscipline,
    discipline_b: MepDiscipline,
    severity: ClashSeverity,
    location_x_mm: i64,
    location_y_mm: i64,
    location_z_mm: i64,
    element_a_id: String,
    element_b_id: String,
    clearance_mm: i32,
    resolved: bool,
    resolution_note: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyIncident {
    incident_id: u64,
    date_epoch: u64,
    severity: IncidentSeverity,
    description: String,
    location_on_site: String,
    injured_worker: String,
    witness_names: Vec<String>,
    corrective_action: String,
    days_lost: u16,
    reported_to_authority: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubcontractorClaim {
    claim_id: u64,
    subcontractor_name: String,
    trade: String,
    period_start_epoch: u64,
    period_end_epoch: u64,
    claimed_amount_cents: u64,
    certified_amount_cents: u64,
    retention_pct: u8,
    status: ClaimStatus,
    line_items: Vec<ClaimLineItem>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClaimLineItem {
    description: String,
    contract_qty: u64,
    completed_qty: u64,
    unit: String,
    rate_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GanttDependency {
    from_milestone_id: u64,
    to_milestone_id: u64,
    lag_days: i16,
    dependency_type: String, // FS, FF, SS, SF
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProjectSchedule {
    project_id: u64,
    project_name: String,
    milestones: Vec<ProjectMilestone>,
    dependencies: Vec<GanttDependency>,
    calendar_start_epoch: u64,
    calendar_end_epoch: u64,
    float_days: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeamDesign {
    beam_id: u64,
    span_mm: u32,
    width_mm: u32,
    depth_mm: u32,
    grade: MaterialGrade,
    top_rebar_count: u8,
    top_rebar_dia_mm: u8,
    bottom_rebar_count: u8,
    bottom_rebar_dia_mm: u8,
    stirrup_dia_mm: u8,
    stirrup_spacing_mm: u16,
    moment_capacity_knm: u32,
    shear_capacity_kn: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColumnDesign {
    column_id: u64,
    height_mm: u32,
    width_mm: u32,
    depth_mm: u32,
    grade: MaterialGrade,
    main_bar_count: u8,
    main_bar_dia_mm: u8,
    tie_dia_mm: u8,
    tie_spacing_mm: u16,
    axial_capacity_kn: u32,
    floor_from: i16,
    floor_to: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlabDesign {
    slab_id: u64,
    thickness_mm: u16,
    grade: MaterialGrade,
    span_x_mm: u32,
    span_y_mm: u32,
    bottom_bar_x_dia: u8,
    bottom_bar_x_spacing: u16,
    bottom_bar_y_dia: u8,
    bottom_bar_y_spacing: u16,
    top_bar_x_dia: u8,
    top_bar_x_spacing: u16,
    top_bar_y_dia: u8,
    top_bar_y_spacing: u16,
    live_load_kpa: u16,
    dead_load_kpa: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityTestRecord {
    test_id: u64,
    test_type: String,
    specimen_id: String,
    date_epoch: u64,
    result_value: u32,
    pass_threshold: u32,
    passed: bool,
    technician: String,
    lab_name: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrawingRevision {
    drawing_number: String,
    revision: String,
    title: String,
    discipline: String,
    date_issued_epoch: u64,
    scale: String,
    sheet_size: String,
    superseded_by: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DailyProgressReport {
    report_id: u64,
    date_epoch: u64,
    weather: WeatherCondition,
    temperature_c: i8,
    wind_speed_kmh: u8,
    manpower_count: u16,
    equipment_hours: u32,
    activities_completed: Vec<String>,
    issues_noted: Vec<String>,
    site_manager: String,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_structural_beam_element() {
    let version = Version::new(1, 0, 0);
    let val = StructuralElement {
        element_id: 1001,
        kind: StructuralElementKind::Beam,
        grade: MaterialGrade::ConcreteC30,
        label: "B-101-L3".into(),
        length_mm: 7200,
        width_mm: 300,
        depth_mm: 600,
        floor_level: 3,
        load_capacity_kn: 450,
        is_critical_path: true,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode beam element");
    let (decoded, decoded_version, _size): (StructuralElement, Version, usize) =
        decode_versioned_value(&bytes).expect("decode beam element");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_structural_column_element() {
    let version = Version::new(1, 1, 0);
    let val = StructuralElement {
        element_id: 2001,
        kind: StructuralElementKind::Column,
        grade: MaterialGrade::ConcreteC40,
        label: "C-A3-L2".into(),
        length_mm: 3600,
        width_mm: 500,
        depth_mm: 500,
        floor_level: 2,
        load_capacity_kn: 2800,
        is_critical_path: false,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode column element");
    let (decoded, decoded_version, _size): (StructuralElement, Version, usize) =
        decode_versioned_value(&bytes).expect("decode column element");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_structural_slab_element() {
    let version = Version::new(1, 0, 2);
    let val = StructuralElement {
        element_id: 3001,
        kind: StructuralElementKind::Slab,
        grade: MaterialGrade::ConcreteC25,
        label: "S-L4-Zone-A".into(),
        length_mm: 8000,
        width_mm: 6000,
        depth_mm: 200,
        floor_level: 4,
        load_capacity_kn: 120,
        is_critical_path: false,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode slab element");
    let (decoded, decoded_version, _size): (StructuralElement, Version, usize) =
        decode_versioned_value(&bytes).expect("decode slab element");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_project_milestone_gantt() {
    let version = Version::new(2, 0, 0);
    let val = ProjectMilestone {
        milestone_id: 10,
        name: "Superstructure Complete Floor 5".into(),
        planned_start_day: 120,
        planned_end_day: 150,
        actual_start_day: 125,
        actual_end_day: 0,
        status: MilestoneStatus::InProgress,
        percent_complete: 60,
        predecessor_ids: vec![8, 9],
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode milestone");
    let (decoded, decoded_version, _size): (ProjectMilestone, Version, usize) =
        decode_versioned_value(&bytes).expect("decode milestone");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_material_takeoff_item() {
    let version = Version::new(1, 0, 0);
    let val = MaterialTakeoffItem {
        item_id: 500,
        description: "Ready-mix concrete C30/37".into(),
        unit: "m3".into(),
        quantity: 245,
        unit_cost_cents: 12500,
        waste_factor_pct: 5,
        supplier: "MegaConcrete Ltd".into(),
        lead_time_days: 3,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode takeoff item");
    let (decoded, decoded_version, _size): (MaterialTakeoffItem, Version, usize) =
        decode_versioned_value(&bytes).expect("decode takeoff item");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_site_inspection_with_checklist() {
    let version = Version::new(1, 2, 0);
    let val = SiteInspection {
        inspection_id: 77,
        project_name: "Harbor Tower Block A".into(),
        date_epoch: 1710500000,
        inspector: "J. Smith, PE".into(),
        weather: WeatherCondition::Overcast,
        entries: vec![
            InspectionChecklistEntry {
                entry_id: 1,
                description: "Formwork alignment check".into(),
                result: InspectionResult::Pass,
                inspector_name: "J. Smith".into(),
                comments: "Within 2mm tolerance".into(),
                photo_count: 4,
            },
            InspectionChecklistEntry {
                entry_id: 2,
                description: "Rebar cover verification".into(),
                result: InspectionResult::ConditionalPass,
                inspector_name: "J. Smith".into(),
                comments: "Cover 38mm, spec requires 40mm, acceptable variance".into(),
                photo_count: 6,
            },
        ],
        overall_result: InspectionResult::ConditionalPass,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode inspection");
    let (decoded, decoded_version, _size): (SiteInspection, Version, usize) =
        decode_versioned_value(&bytes).expect("decode inspection");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_change_order_record() {
    let version = Version::new(1, 0, 0);
    let val = ChangeOrder {
        co_number: 14,
        title: "Additional pile caps at grid F-G".into(),
        description: "Geotechnical report update requires 4 additional pile caps".into(),
        status: ChangeOrderStatus::Approved,
        requested_by: "Structural Engineer".into(),
        cost_impact_cents: 4_500_000,
        schedule_impact_days: 12,
        affected_elements: vec![1001, 1002, 1003, 1004],
        date_submitted_epoch: 1709800000,
        date_resolved_epoch: 1710200000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode change order");
    let (decoded, decoded_version, _size): (ChangeOrder, Version, usize) =
        decode_versioned_value(&bytes).expect("decode change order");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_rfi_log_entry() {
    let version = Version::new(1, 3, 0);
    let val = RfiRecord {
        rfi_number: 42,
        subject: "Waterproofing detail at expansion joint EJ-03".into(),
        question: "Drawing A-301 shows membrane termination at top of wall but structural drawing S-201 shows a rebate. Which takes precedence?".into(),
        answer: "Follow detail on S-201 Rev C. Membrane to be tucked into rebate. See SK-042 for clarification.".into(),
        priority: RfiPriority::High,
        status: RfiStatus::Closed,
        submitted_by: "Waterproofing Subcontractor".into(),
        assigned_to: "Architect".into(),
        date_opened_epoch: 1709000000,
        date_closed_epoch: 1709400000,
        related_drawing_refs: vec![
            "A-301".into(),
            "S-201".into(),
            "SK-042".into(),
        ],
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode rfi");
    let (decoded, decoded_version, _size): (RfiRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode rfi");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_concrete_mix_design() {
    let version = Version::new(2, 1, 0);
    let val = ConcreteMixDesign {
        mix_id: 301,
        designation: "C40/50-S4-XC4".into(),
        target_strength_mpa: 50,
        water_cement_ratio: 420, // 0.42
        cement_kg_per_m3: 380,
        water_kg_per_m3: 160,
        fine_aggregate_kg: 720,
        coarse_aggregate_kg: 1080,
        admixture_ml_per_m3: 3800,
        slump_mm: 180,
        max_aggregate_size_mm: 20,
        placement_method: ConcretePlacement::PumpedInPlace,
        air_content_pct_x10: 45, // 4.5%
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode mix design");
    let (decoded, decoded_version, _size): (ConcreteMixDesign, Version, usize) =
        decode_versioned_value(&bytes).expect("decode mix design");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_rebar_schedule_entry() {
    let version = Version::new(1, 0, 0);
    let val = RebarScheduleEntry {
        bar_mark: "BM-101".into(),
        grade: RebarGrade::Grade60,
        diameter_mm: 25,
        length_mm: 6800,
        quantity: 24,
        shape_code: "38".into(),
        bending_dimension_a_mm: 400,
        bending_dimension_b_mm: 150,
        element_ref: "B-101-L3".into(),
        total_weight_grams: 24 * 3854 * 68 / 10, // approximate
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode rebar schedule");
    let (decoded, decoded_version, _size): (RebarScheduleEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode rebar schedule");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_mep_clash_detection_result() {
    let version = Version::new(1, 0, 0);
    let val = MepClashResult {
        clash_id: 8899,
        discipline_a: MepDiscipline::Hvac,
        discipline_b: MepDiscipline::Electrical,
        severity: ClashSeverity::Major,
        location_x_mm: 15200,
        location_y_mm: 8300,
        location_z_mm: 3050,
        element_a_id: "DUCT-AHU3-BR2".into(),
        element_b_id: "CABLE-TRAY-CT-14".into(),
        clearance_mm: -45,
        resolved: false,
        resolution_note: String::new(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode clash result");
    let (decoded, decoded_version, _size): (MepClashResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode clash result");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_mep_clash_resolved() {
    let version = Version::new(1, 1, 0);
    let val = MepClashResult {
        clash_id: 8900,
        discipline_a: MepDiscipline::Plumbing,
        discipline_b: MepDiscipline::FireProtection,
        severity: ClashSeverity::Minor,
        location_x_mm: 22400,
        location_y_mm: 12100,
        location_z_mm: 2800,
        element_a_id: "PIPE-CW-32".into(),
        element_b_id: "SPRINKLER-SP-L3-07".into(),
        clearance_mm: 5,
        resolved: true,
        resolution_note: "Rerouted CW pipe 100mm south per RFI-55".into(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode resolved clash");
    let (decoded, decoded_version, _size): (MepClashResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode resolved clash");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_safety_incident_report() {
    let version = Version::new(1, 0, 0);
    let val = SafetyIncident {
        incident_id: 201,
        date_epoch: 1710300000,
        severity: IncidentSeverity::FirstAid,
        description: "Worker sustained minor cut while stripping formwork on L4".into(),
        location_on_site: "Level 4, Grid B3-C3".into(),
        injured_worker: "Worker #4421".into(),
        witness_names: vec!["Foreman T. Lee".into(), "Worker #4418".into()],
        corrective_action: "Issued cut-resistant gloves to all formwork crew. Updated TBT".into(),
        days_lost: 0,
        reported_to_authority: false,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode safety incident");
    let (decoded, decoded_version, _size): (SafetyIncident, Version, usize) =
        decode_versioned_value(&bytes).expect("decode safety incident");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_subcontractor_progress_claim() {
    let version = Version::new(2, 0, 0);
    let val = SubcontractorClaim {
        claim_id: 55,
        subcontractor_name: "Pacific Rebar Services".into(),
        trade: "Reinforcement Steel".into(),
        period_start_epoch: 1709200000,
        period_end_epoch: 1710400000,
        claimed_amount_cents: 85_000_00,
        certified_amount_cents: 78_500_00,
        retention_pct: 5,
        status: ClaimStatus::Certified,
        line_items: vec![
            ClaimLineItem {
                description: "Supply and fix rebar - columns L3".into(),
                contract_qty: 12000,
                completed_qty: 11800,
                unit: "kg".into(),
                rate_cents: 350,
            },
            ClaimLineItem {
                description: "Supply and fix rebar - beams L3".into(),
                contract_qty: 18000,
                completed_qty: 14200,
                unit: "kg".into(),
                rate_cents: 380,
            },
        ],
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode subcontractor claim");
    let (decoded, decoded_version, _size): (SubcontractorClaim, Version, usize) =
        decode_versioned_value(&bytes).expect("decode subcontractor claim");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_project_schedule_with_dependencies() {
    let version = Version::new(3, 0, 0);
    let val = ProjectSchedule {
        project_id: 1,
        project_name: "Downtown Mixed-Use Tower".into(),
        milestones: vec![
            ProjectMilestone {
                milestone_id: 1,
                name: "Excavation Complete".into(),
                planned_start_day: 1,
                planned_end_day: 30,
                actual_start_day: 1,
                actual_end_day: 28,
                status: MilestoneStatus::Completed,
                percent_complete: 100,
                predecessor_ids: vec![],
            },
            ProjectMilestone {
                milestone_id: 2,
                name: "Foundation Piling".into(),
                planned_start_day: 25,
                planned_end_day: 60,
                actual_start_day: 26,
                actual_end_day: 0,
                status: MilestoneStatus::InProgress,
                percent_complete: 70,
                predecessor_ids: vec![1],
            },
        ],
        dependencies: vec![GanttDependency {
            from_milestone_id: 1,
            to_milestone_id: 2,
            lag_days: -5,
            dependency_type: "FS".into(),
        }],
        calendar_start_epoch: 1704067200,
        calendar_end_epoch: 1735689600,
        float_days: 14,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode project schedule");
    let (decoded, decoded_version, _size): (ProjectSchedule, Version, usize) =
        decode_versioned_value(&bytes).expect("decode project schedule");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_beam_design_specification() {
    let version = Version::new(1, 0, 0);
    let val = BeamDesign {
        beam_id: 4001,
        span_mm: 7200,
        width_mm: 300,
        depth_mm: 600,
        grade: MaterialGrade::ConcreteC40,
        top_rebar_count: 3,
        top_rebar_dia_mm: 20,
        bottom_rebar_count: 4,
        bottom_rebar_dia_mm: 25,
        stirrup_dia_mm: 10,
        stirrup_spacing_mm: 150,
        moment_capacity_knm: 520,
        shear_capacity_kn: 280,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode beam design");
    let (decoded, decoded_version, _size): (BeamDesign, Version, usize) =
        decode_versioned_value(&bytes).expect("decode beam design");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_column_design_specification() {
    let version = Version::new(1, 0, 0);
    let val = ColumnDesign {
        column_id: 5001,
        height_mm: 3600,
        width_mm: 600,
        depth_mm: 600,
        grade: MaterialGrade::ConcreteC50,
        main_bar_count: 12,
        main_bar_dia_mm: 32,
        tie_dia_mm: 10,
        tie_spacing_mm: 200,
        axial_capacity_kn: 8500,
        floor_from: 0,
        floor_to: 3,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode column design");
    let (decoded, decoded_version, _size): (ColumnDesign, Version, usize) =
        decode_versioned_value(&bytes).expect("decode column design");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_slab_design_specification() {
    let version = Version::new(1, 0, 0);
    let val = SlabDesign {
        slab_id: 6001,
        thickness_mm: 200,
        grade: MaterialGrade::ConcreteC30,
        span_x_mm: 8000,
        span_y_mm: 6000,
        bottom_bar_x_dia: 12,
        bottom_bar_x_spacing: 150,
        bottom_bar_y_dia: 12,
        bottom_bar_y_spacing: 200,
        top_bar_x_dia: 10,
        top_bar_x_spacing: 200,
        top_bar_y_dia: 10,
        top_bar_y_spacing: 250,
        live_load_kpa: 50, // 5.0 kPa
        dead_load_kpa: 65, // 6.5 kPa including self-weight
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode slab design");
    let (decoded, decoded_version, _size): (SlabDesign, Version, usize) =
        decode_versioned_value(&bytes).expect("decode slab design");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_quality_test_record_concrete_cube() {
    let version = Version::new(1, 0, 0);
    let val = QualityTestRecord {
        test_id: 7701,
        test_type: "Cube Crushing Strength 28-day".into(),
        specimen_id: "CUBE-2024-0312-A".into(),
        date_epoch: 1710500000,
        result_value: 42,   // MPa
        pass_threshold: 40, // MPa
        passed: true,
        technician: "Lab Tech R. Kumar".into(),
        lab_name: "CentralTest Laboratories".into(),
        notes: "3 specimens tested, mean 42.3 MPa, min 41.1 MPa".into(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode quality test");
    let (decoded, decoded_version, _size): (QualityTestRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode quality test");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_drawing_revision_record() {
    let version = Version::new(1, 0, 0);
    let val = DrawingRevision {
        drawing_number: "S-201".into(),
        revision: "C".into(),
        title: "Structural Plan Level 2".into(),
        discipline: "Structural".into(),
        date_issued_epoch: 1709800000,
        scale: "1:50".into(),
        sheet_size: "A1".into(),
        superseded_by: String::new(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode drawing revision");
    let (decoded, decoded_version, _size): (DrawingRevision, Version, usize) =
        decode_versioned_value(&bytes).expect("decode drawing revision");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_daily_progress_report() {
    let version = Version::new(1, 0, 0);
    let val = DailyProgressReport {
        report_id: 312,
        date_epoch: 1710400000,
        weather: WeatherCondition::Clear,
        temperature_c: 28,
        wind_speed_kmh: 12,
        manpower_count: 145,
        equipment_hours: 480,
        activities_completed: vec![
            "L4 slab formwork erection 60% complete".into(),
            "L3 column rebar tying completed".into(),
            "MEP rough-in L2 corridor zone B".into(),
        ],
        issues_noted: vec!["Concrete pump breakdown 2hr delay on L3 pour".into()],
        site_manager: "Eng. M. Tanaka".into(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode daily report");
    let (decoded, decoded_version, _size): (DailyProgressReport, Version, usize) =
        decode_versioned_value(&bytes).expect("decode daily report");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_change_order_rejected() {
    let version = Version::new(1, 0, 1);
    let val = ChangeOrder {
        co_number: 19,
        title: "Upgrade lobby flooring to imported marble".into(),
        description: "Client requests upgrade from porcelain tile to Carrara marble in main lobby and elevator lobbies L1-L3".into(),
        status: ChangeOrderStatus::Rejected,
        requested_by: "Interior Designer".into(),
        cost_impact_cents: 12_800_000,
        schedule_impact_days: 45,
        affected_elements: vec![],
        date_submitted_epoch: 1710100000,
        date_resolved_epoch: 1710300000,
    };
    let bytes =
        encode_versioned_value(&val, version.clone()).expect("encode rejected change order");
    let (decoded, decoded_version, _size): (ChangeOrder, Version, usize) =
        decode_versioned_value(&bytes).expect("decode rejected change order");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}
