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

// --- Domain types: Aviation Maintenance, Repair, Overhaul (MRO) ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AircraftLogbookEntry {
    entry_id: u64,
    aircraft_reg: String,
    date_utc: String,
    flight_hours: f64,
    cycles: u32,
    discrepancy: Option<String>,
    corrective_action: Option<String>,
    mechanic_license: String,
    inspector_license: Option<String>,
    ata_chapter: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AirworthinessDirective {
    ad_number: String,
    title: String,
    effective_date: String,
    applicability: Vec<String>,
    compliance_method: String,
    recurring: bool,
    interval_hours: Option<f64>,
    interval_cycles: Option<u32>,
    interval_calendar_days: Option<u32>,
    terminating_action: Option<String>,
    status: AdStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AdStatus {
    Open,
    Complied,
    Terminated,
    NotApplicable,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ServiceBulletin {
    sb_number: String,
    issuer: String,
    revision: u8,
    category: SbCategory,
    title: String,
    effectivity_range: Vec<String>,
    estimated_manhours: f64,
    parts_required: Vec<SbPartRef>,
    mandatory: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SbCategory {
    Mandatory,
    Alert,
    Optional,
    Informational,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SbPartRef {
    part_number: String,
    quantity: u32,
    unit_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EnginePerfTrend {
    engine_serial: String,
    flight_id: u64,
    timestamp_epoch: u64,
    egt_celsius: f64,
    n1_percent: f64,
    n2_percent: f64,
    fuel_flow_kg_per_hr: f64,
    oil_pressure_psi: f64,
    oil_temp_celsius: f64,
    vibration_ips: f64,
    bleed_air_status: bool,
    altitude_ft: u32,
    mach_number: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LandingGearCycleTracker {
    gear_position: GearPosition,
    serial_number: String,
    total_cycles: u64,
    cycles_since_overhaul: u32,
    overhaul_limit_cycles: u32,
    last_inspection_date: String,
    tire_landings: u32,
    brake_wear_percent: f64,
    shimmy_damper_condition: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum GearPosition {
    NoseLandingGear,
    LeftMainGear,
    RightMainGear,
    CenterMainGear,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ComponentTso {
    component_name: String,
    part_number: String,
    serial_number: String,
    hours_since_overhaul: f64,
    cycles_since_overhaul: u32,
    overhaul_interval_hours: f64,
    overhaul_interval_cycles: u32,
    time_remaining_hours: f64,
    installed_position: String,
    aircraft_reg: String,
    condition: ComponentCondition,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ComponentCondition {
    Serviceable,
    Unserviceable,
    Overhauled,
    Repaired,
    Inspected,
    New,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MelDeferral {
    mel_item_number: String,
    ata_chapter: u16,
    category: MelCategory,
    description: String,
    operational_restrictions: Vec<String>,
    rectification_interval_days: u32,
    deferral_date: String,
    expiry_date: String,
    aircraft_reg: String,
    placard_installed: bool,
    maintenance_procedure_ref: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MelCategory {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BorescopeReport {
    report_id: u64,
    engine_serial: String,
    inspection_date: String,
    inspector_id: String,
    stages_inspected: Vec<BorescopeStage>,
    overall_assessment: String,
    next_inspection_hours: f64,
    images_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BorescopeStage {
    stage_name: String,
    blade_count: u16,
    findings: Vec<BorescopeFinding>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BorescopeFinding {
    blade_number: u16,
    finding_type: String,
    severity: FindingSeverity,
    dimension_mm: Option<f64>,
    serviceable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FindingSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NdtResult {
    report_number: String,
    technique: NdtTechnique,
    part_number: String,
    serial_number: String,
    test_date: String,
    technician_cert: String,
    calibration_block_id: String,
    sensitivity_db: f64,
    indications: Vec<NdtIndication>,
    accept_reject: AcceptReject,
    reference_standard: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum NdtTechnique {
    UltrasonicPulseEcho,
    EddyCurrent,
    MagneticParticle,
    DyePenetrant,
    Radiographic,
    ThermographicInfrared,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NdtIndication {
    location_desc: String,
    length_mm: f64,
    depth_mm: Option<f64>,
    indication_type: String,
    relevant: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AcceptReject {
    Accept,
    Reject,
    ConditionalAccept { next_inspection_hours: f64 },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PartsInventoryItem {
    part_number: String,
    description: String,
    category: PartCategory,
    quantity_on_hand: u32,
    minimum_stock: u32,
    unit_price_cents: u64,
    shelf_life_days: Option<u32>,
    certification: String,
    warehouse_location: String,
    last_receipt_date: String,
    vendor_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PartCategory {
    Rotable,
    Expendable,
    Consumable,
    StandardHardware,
    RawMaterial,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WorkOrder {
    wo_number: String,
    aircraft_reg: String,
    work_type: WorkType,
    priority: WoPriority,
    description: String,
    task_cards: Vec<TaskCard>,
    estimated_manhours: f64,
    actual_manhours: Option<f64>,
    status: WoStatus,
    open_date: String,
    due_date: String,
    close_date: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum WorkType {
    Scheduled,
    Unscheduled,
    AdCompliance,
    SbIncorporation,
    Modification,
    HeavyMaintenance,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum WoPriority {
    Aog,
    Critical,
    High,
    Routine,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum WoStatus {
    Open,
    InProgress,
    PendingParts,
    PendingInspection,
    Completed,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TaskCard {
    task_number: String,
    zone: String,
    skill_type: String,
    estimated_hours: f64,
    completed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReliabilityAnalysis {
    aircraft_type: String,
    period_start: String,
    period_end: String,
    dispatch_reliability_pct: f64,
    total_flights: u64,
    delays: u32,
    cancellations: u32,
    pirep_count: u32,
    pirep_rate_per_1000fh: f64,
    top_ata_chapters: Vec<AtaReliabilityEntry>,
    fleet_avg_daily_utilization_hrs: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AtaReliabilityEntry {
    ata_chapter: u16,
    description: String,
    event_count: u32,
    rate_per_1000fh: f64,
    trend: TrendDirection,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    InsufficientData,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FuelContaminationTest {
    sample_id: String,
    aircraft_reg: String,
    sample_point: FuelSamplePoint,
    test_date: String,
    water_content_ppm: f64,
    particulate_mg_per_l: f64,
    microbial_cfu_per_ml: u32,
    conductivity_ps_per_m: f64,
    appearance: String,
    pass: bool,
    lab_cert_number: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FuelSamplePoint {
    SumpDrain,
    TankLow,
    FilterBowl,
    FuelTruck,
    StorageTank,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AvionicsSwConfig {
    lru_name: String,
    part_number: String,
    software_part_number: String,
    software_version: String,
    dal_level: DalLevel,
    configuration_table: Vec<SwConfigParam>,
    last_load_date: String,
    aircraft_reg: String,
    approved_by: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DalLevel {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SwConfigParam {
    parameter_name: String,
    value: String,
    default_value: String,
    editable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MaintenanceForecast {
    aircraft_reg: String,
    forecast_date: String,
    current_flight_hours: f64,
    current_cycles: u32,
    daily_utilization_hrs: f64,
    daily_cycles: f64,
    upcoming_tasks: Vec<ForecastTask>,
    next_heavy_check_type: String,
    next_heavy_check_due_date: String,
    estimated_downtime_days: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ForecastTask {
    task_description: String,
    due_at_hours: f64,
    due_at_cycles: Option<u32>,
    estimated_due_date: String,
    source: String,
    can_package: bool,
}

// --- Tests ---

#[test]
fn test_aircraft_logbook_entry_roundtrip() {
    let cfg = config::standard();
    let entry = AircraftLogbookEntry {
        entry_id: 10042,
        aircraft_reg: "N737WN".to_string(),
        date_utc: "2026-03-14T18:30:00Z".to_string(),
        flight_hours: 42567.3,
        cycles: 31024,
        discrepancy: Some(
            "Cabin altitude warning light intermittent during cruise FL350".to_string(),
        ),
        corrective_action: Some(
            "Replaced outflow valve controller P/N 103-2847, tested operational".to_string(),
        ),
        mechanic_license: "AP-12345678".to_string(),
        inspector_license: Some("IA-87654321".to_string()),
        ata_chapter: 21,
    };
    let encoded = encode_to_vec(&entry, cfg).expect("encode logbook entry");
    let (decoded, _): (AircraftLogbookEntry, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode logbook entry");
    assert_eq!(entry, decoded);
}

#[test]
fn test_airworthiness_directive_roundtrip() {
    let cfg = config::standard();
    let ad = AirworthinessDirective {
        ad_number: "2026-04-12".to_string(),
        title: "Wing Spar Inspection for Fatigue Cracking".to_string(),
        effective_date: "2026-05-01".to_string(),
        applicability: vec![
            "Boeing 737-700 S/N 28000-32000".to_string(),
            "Boeing 737-800 S/N 30000-35000".to_string(),
        ],
        compliance_method: "Detailed visual inspection per Boeing SB 737-57-1320".to_string(),
        recurring: true,
        interval_hours: Some(6000.0),
        interval_cycles: Some(4000),
        interval_calendar_days: None,
        terminating_action: Some("Replacement of wing spar chord per SB 737-57-1321".to_string()),
        status: AdStatus::Open,
    };
    let encoded = encode_to_vec(&ad, cfg).expect("encode AD");
    let (decoded, _): (AirworthinessDirective, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AD");
    assert_eq!(ad, decoded);
}

#[test]
fn test_service_bulletin_roundtrip() {
    let cfg = config::standard();
    let sb = ServiceBulletin {
        sb_number: "CFM56-72-0987".to_string(),
        issuer: "CFM International".to_string(),
        revision: 3,
        category: SbCategory::Alert,
        title: "HPT blade coating inspection and rework".to_string(),
        effectivity_range: vec![
            "ESN 725100 through 725500".to_string(),
            "ESN 726000 through 726200".to_string(),
        ],
        estimated_manhours: 48.0,
        parts_required: vec![
            SbPartRef {
                part_number: "1860M47G03".to_string(),
                quantity: 92,
                unit_cost_cents: 145000,
            },
            SbPartRef {
                part_number: "9386M10P01".to_string(),
                quantity: 1,
                unit_cost_cents: 3200000,
            },
        ],
        mandatory: false,
    };
    let encoded = encode_to_vec(&sb, cfg).expect("encode SB");
    let (decoded, _): (ServiceBulletin, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode SB");
    assert_eq!(sb, decoded);
}

#[test]
fn test_engine_performance_trending_roundtrip() {
    let cfg = config::standard();
    let trend = EnginePerfTrend {
        engine_serial: "ESN-725432".to_string(),
        flight_id: 20260314001,
        timestamp_epoch: 1773696000,
        egt_celsius: 842.5,
        n1_percent: 91.3,
        n2_percent: 96.7,
        fuel_flow_kg_per_hr: 2340.0,
        oil_pressure_psi: 45.2,
        oil_temp_celsius: 112.8,
        vibration_ips: 1.2,
        bleed_air_status: true,
        altitude_ft: 35000,
        mach_number: 0.78,
    };
    let encoded = encode_to_vec(&trend, cfg).expect("encode engine trend");
    let (decoded, _): (EnginePerfTrend, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode engine trend");
    assert_eq!(trend, decoded);
}

#[test]
fn test_landing_gear_cycle_tracker_roundtrip() {
    let cfg = config::standard();
    let gear = LandingGearCycleTracker {
        gear_position: GearPosition::NoseLandingGear,
        serial_number: "NLG-2024-0042".to_string(),
        total_cycles: 28500,
        cycles_since_overhaul: 8200,
        overhaul_limit_cycles: 20000,
        last_inspection_date: "2026-02-15".to_string(),
        tire_landings: 312,
        brake_wear_percent: 34.5,
        shimmy_damper_condition: "Serviceable - minor seepage observed".to_string(),
    };
    let encoded = encode_to_vec(&gear, cfg).expect("encode landing gear");
    let (decoded, _): (LandingGearCycleTracker, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode landing gear");
    assert_eq!(gear, decoded);
}

#[test]
fn test_component_tso_roundtrip() {
    let cfg = config::standard();
    let comp = ComponentTso {
        component_name: "Integrated Drive Generator".to_string(),
        part_number: "976J240-3".to_string(),
        serial_number: "IDG-44210".to_string(),
        hours_since_overhaul: 4520.3,
        cycles_since_overhaul: 3100,
        overhaul_interval_hours: 12000.0,
        overhaul_interval_cycles: 8000,
        time_remaining_hours: 7479.7,
        installed_position: "Engine 1 Accessory Gearbox".to_string(),
        aircraft_reg: "N814UA".to_string(),
        condition: ComponentCondition::Overhauled,
    };
    let encoded = encode_to_vec(&comp, cfg).expect("encode component TSO");
    let (decoded, _): (ComponentTso, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode component TSO");
    assert_eq!(comp, decoded);
}

#[test]
fn test_mel_deferral_roundtrip() {
    let cfg = config::standard();
    let mel = MelDeferral {
        mel_item_number: "34-11-01".to_string(),
        ata_chapter: 34,
        category: MelCategory::C,
        description: "Navigation Display #2 (FO side) - color anomaly on weather radar overlay"
            .to_string(),
        operational_restrictions: vec![
            "Captain must verify weather via Display #1 prior to entering convective areas"
                .to_string(),
            "ACARS weather uplink required".to_string(),
        ],
        rectification_interval_days: 10,
        deferral_date: "2026-03-12".to_string(),
        expiry_date: "2026-03-22".to_string(),
        aircraft_reg: "C-FXYZ".to_string(),
        placard_installed: true,
        maintenance_procedure_ref: Some("AMM 34-11-01-400-001".to_string()),
    };
    let encoded = encode_to_vec(&mel, cfg).expect("encode MEL deferral");
    let (decoded, _): (MelDeferral, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode MEL deferral");
    assert_eq!(mel, decoded);
}

#[test]
fn test_borescope_report_roundtrip() {
    let cfg = config::standard();
    let report = BorescopeReport {
        report_id: 56789,
        engine_serial: "ESN-P1042A".to_string(),
        inspection_date: "2026-03-10".to_string(),
        inspector_id: "NDT-LEVEL3-007".to_string(),
        stages_inspected: vec![
            BorescopeStage {
                stage_name: "HPT Stage 1".to_string(),
                blade_count: 68,
                findings: vec![
                    BorescopeFinding {
                        blade_number: 14,
                        finding_type: "Tip erosion".to_string(),
                        severity: FindingSeverity::Minor,
                        dimension_mm: Some(0.8),
                        serviceable: true,
                    },
                    BorescopeFinding {
                        blade_number: 42,
                        finding_type: "Leading edge nick".to_string(),
                        severity: FindingSeverity::Moderate,
                        dimension_mm: Some(1.5),
                        serviceable: true,
                    },
                ],
            },
            BorescopeStage {
                stage_name: "HPT Stage 2".to_string(),
                blade_count: 72,
                findings: vec![
                    BorescopeFinding {
                        blade_number: 55,
                        finding_type: "Coating spallation".to_string(),
                        severity: FindingSeverity::Major,
                        dimension_mm: Some(4.2),
                        serviceable: false,
                    },
                ],
            },
        ],
        overall_assessment: "Engine serviceable with monitoring; Stage 2 blade 55 requires replacement at next shop visit".to_string(),
        next_inspection_hours: 500.0,
        images_count: 142,
    };
    let encoded = encode_to_vec(&report, cfg).expect("encode borescope report");
    let (decoded, _): (BorescopeReport, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode borescope report");
    assert_eq!(report, decoded);
}

#[test]
fn test_ndt_ultrasonic_result_roundtrip() {
    let cfg = config::standard();
    let result = NdtResult {
        report_number: "UT-2026-0314-001".to_string(),
        technique: NdtTechnique::UltrasonicPulseEcho,
        part_number: "65C30524-7".to_string(),
        serial_number: "SN-2019-4421".to_string(),
        test_date: "2026-03-14".to_string(),
        technician_cert: "UT-ASNT-III-2847".to_string(),
        calibration_block_id: "IIW-V1-2024".to_string(),
        sensitivity_db: 42.0,
        indications: vec![NdtIndication {
            location_desc: "Fwd spar cap, STA 412, 2 inches aft of fastener row".to_string(),
            length_mm: 3.2,
            depth_mm: Some(1.1),
            indication_type: "Linear".to_string(),
            relevant: true,
        }],
        accept_reject: AcceptReject::ConditionalAccept {
            next_inspection_hours: 2000.0,
        },
        reference_standard: "AMS 2630 Rev C".to_string(),
    };
    let encoded = encode_to_vec(&result, cfg).expect("encode NDT result");
    let (decoded, _): (NdtResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode NDT result");
    assert_eq!(result, decoded);
}

#[test]
fn test_ndt_eddy_current_roundtrip() {
    let cfg = config::standard();
    let result = NdtResult {
        report_number: "EC-2026-0310-005".to_string(),
        technique: NdtTechnique::EddyCurrent,
        part_number: "332A1200-11".to_string(),
        serial_number: "WHL-7742".to_string(),
        test_date: "2026-03-10".to_string(),
        technician_cert: "ET-ASNT-II-1193".to_string(),
        calibration_block_id: "EC-REF-2025-A".to_string(),
        sensitivity_db: 0.0,
        indications: vec![],
        accept_reject: AcceptReject::Accept,
        reference_standard: "ASTM E2884".to_string(),
    };
    let encoded = encode_to_vec(&result, cfg).expect("encode EC NDT");
    let (decoded, _): (NdtResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EC NDT");
    assert_eq!(result, decoded);
}

#[test]
fn test_parts_inventory_rotable_roundtrip() {
    let cfg = config::standard();
    let item = PartsInventoryItem {
        part_number: "65-73906-27".to_string(),
        description: "Hydraulic Pump - Engine Driven (System A)".to_string(),
        category: PartCategory::Rotable,
        quantity_on_hand: 3,
        minimum_stock: 2,
        unit_price_cents: 8750000,
        shelf_life_days: None,
        certification: "FAA 8130-3 / EASA Form 1".to_string(),
        warehouse_location: "HAZ-A-12-04".to_string(),
        last_receipt_date: "2026-01-20".to_string(),
        vendor_codes: vec!["V-PARKER".to_string(), "V-EATON".to_string()],
    };
    let encoded = encode_to_vec(&item, cfg).expect("encode rotable part");
    let (decoded, _): (PartsInventoryItem, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode rotable part");
    assert_eq!(item, decoded);
}

#[test]
fn test_parts_inventory_consumable_roundtrip() {
    let cfg = config::standard();
    let item = PartsInventoryItem {
        part_number: "BMS5-95-TYPE-II".to_string(),
        description: "Sealant, polysulfide, fuel tank, Class B".to_string(),
        category: PartCategory::Consumable,
        quantity_on_hand: 48,
        minimum_stock: 20,
        unit_price_cents: 12500,
        shelf_life_days: Some(365),
        certification: "QPL BMS5-95".to_string(),
        warehouse_location: "CHEM-B-03-01".to_string(),
        last_receipt_date: "2026-02-28".to_string(),
        vendor_codes: vec!["V-PPG".to_string(), "V-FLAMEMASTER".to_string()],
    };
    let encoded = encode_to_vec(&item, cfg).expect("encode consumable part");
    let (decoded, _): (PartsInventoryItem, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode consumable part");
    assert_eq!(item, decoded);
}

#[test]
fn test_work_order_scheduled_roundtrip() {
    let cfg = config::standard();
    let wo = WorkOrder {
        wo_number: "WO-2026-MAR-0042".to_string(),
        aircraft_reg: "N321DL".to_string(),
        work_type: WorkType::Scheduled,
        priority: WoPriority::Routine,
        description: "C-Check Phase 1 - Structural inspection and corrosion treatment".to_string(),
        task_cards: vec![
            TaskCard {
                task_number: "53-10-11-210-001".to_string(),
                zone: "Zone 131 - Fuselage lower lobe".to_string(),
                skill_type: "Structures".to_string(),
                estimated_hours: 8.0,
                completed: true,
            },
            TaskCard {
                task_number: "53-10-11-210-002".to_string(),
                zone: "Zone 132 - Fuselage lower lobe fwd".to_string(),
                skill_type: "Structures".to_string(),
                estimated_hours: 6.5,
                completed: false,
            },
            TaskCard {
                task_number: "72-00-00-780-001".to_string(),
                zone: "Zone 411 - Engine #1".to_string(),
                skill_type: "Powerplant".to_string(),
                estimated_hours: 4.0,
                completed: false,
            },
        ],
        estimated_manhours: 340.0,
        actual_manhours: None,
        status: WoStatus::InProgress,
        open_date: "2026-03-01".to_string(),
        due_date: "2026-03-20".to_string(),
        close_date: None,
    };
    let encoded = encode_to_vec(&wo, cfg).expect("encode work order");
    let (decoded, _): (WorkOrder, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode work order");
    assert_eq!(wo, decoded);
}

#[test]
fn test_work_order_aog_roundtrip() {
    let cfg = config::standard();
    let wo = WorkOrder {
        wo_number: "WO-2026-MAR-AOG-003".to_string(),
        aircraft_reg: "D-AIPH".to_string(),
        work_type: WorkType::Unscheduled,
        priority: WoPriority::Aog,
        description: "Engine 2 high vibration event during takeoff roll - rejected takeoff"
            .to_string(),
        task_cards: vec![TaskCard {
            task_number: "72-00-00-810-AOG".to_string(),
            zone: "Zone 421 - Engine #2".to_string(),
            skill_type: "Powerplant".to_string(),
            estimated_hours: 16.0,
            completed: true,
        }],
        estimated_manhours: 24.0,
        actual_manhours: Some(22.5),
        status: WoStatus::Completed,
        open_date: "2026-03-13".to_string(),
        due_date: "2026-03-14".to_string(),
        close_date: Some("2026-03-14".to_string()),
    };
    let encoded = encode_to_vec(&wo, cfg).expect("encode AOG work order");
    let (decoded, _): (WorkOrder, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AOG work order");
    assert_eq!(wo, decoded);
}

#[test]
fn test_reliability_analysis_roundtrip() {
    let cfg = config::standard();
    let analysis = ReliabilityAnalysis {
        aircraft_type: "Boeing 737-800".to_string(),
        period_start: "2026-01-01".to_string(),
        period_end: "2026-03-01".to_string(),
        dispatch_reliability_pct: 99.12,
        total_flights: 14200,
        delays: 98,
        cancellations: 27,
        pirep_count: 342,
        pirep_rate_per_1000fh: 8.4,
        top_ata_chapters: vec![
            AtaReliabilityEntry {
                ata_chapter: 32,
                description: "Landing Gear".to_string(),
                event_count: 45,
                rate_per_1000fh: 1.1,
                trend: TrendDirection::Stable,
            },
            AtaReliabilityEntry {
                ata_chapter: 21,
                description: "Air Conditioning".to_string(),
                event_count: 38,
                rate_per_1000fh: 0.93,
                trend: TrendDirection::Improving,
            },
            AtaReliabilityEntry {
                ata_chapter: 34,
                description: "Navigation".to_string(),
                event_count: 31,
                rate_per_1000fh: 0.76,
                trend: TrendDirection::Degrading,
            },
        ],
        fleet_avg_daily_utilization_hrs: 10.8,
    };
    let encoded = encode_to_vec(&analysis, cfg).expect("encode reliability analysis");
    let (decoded, _): (ReliabilityAnalysis, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reliability analysis");
    assert_eq!(analysis, decoded);
}

#[test]
fn test_fuel_contamination_pass_roundtrip() {
    let cfg = config::standard();
    let test_result = FuelContaminationTest {
        sample_id: "FUEL-2026-0314-A".to_string(),
        aircraft_reg: "JA804A".to_string(),
        sample_point: FuelSamplePoint::SumpDrain,
        test_date: "2026-03-14".to_string(),
        water_content_ppm: 12.3,
        particulate_mg_per_l: 0.4,
        microbial_cfu_per_ml: 0,
        conductivity_ps_per_m: 285.0,
        appearance: "Clear and bright, no visible particulate or free water".to_string(),
        pass: true,
        lab_cert_number: "ISO17025-FUEL-2024-0891".to_string(),
    };
    let encoded = encode_to_vec(&test_result, cfg).expect("encode fuel test pass");
    let (decoded, _): (FuelContaminationTest, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode fuel test pass");
    assert_eq!(test_result, decoded);
}

#[test]
fn test_fuel_contamination_fail_roundtrip() {
    let cfg = config::standard();
    let test_result = FuelContaminationTest {
        sample_id: "FUEL-2026-0312-C".to_string(),
        aircraft_reg: "VH-OQA".to_string(),
        sample_point: FuelSamplePoint::TankLow,
        test_date: "2026-03-12".to_string(),
        water_content_ppm: 85.7,
        particulate_mg_per_l: 3.2,
        microbial_cfu_per_ml: 1200,
        conductivity_ps_per_m: 410.0,
        appearance: "Hazy with visible sediment, faint sulfurous odor".to_string(),
        pass: false,
        lab_cert_number: "ISO17025-FUEL-2024-0891".to_string(),
    };
    let encoded = encode_to_vec(&test_result, cfg).expect("encode fuel test fail");
    let (decoded, _): (FuelContaminationTest, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode fuel test fail");
    assert_eq!(test_result, decoded);
}

#[test]
fn test_avionics_sw_config_roundtrip() {
    let cfg = config::standard();
    let sw = AvionicsSwConfig {
        lru_name: "Flight Management Computer".to_string(),
        part_number: "822-1893-402".to_string(),
        software_part_number: "822-1893-402-SW".to_string(),
        software_version: "U14.2A".to_string(),
        dal_level: DalLevel::A,
        configuration_table: vec![
            SwConfigParam {
                parameter_name: "NAV_DATABASE_CYCLE".to_string(),
                value: "AIRAC 2603".to_string(),
                default_value: "AIRAC 2601".to_string(),
                editable: true,
            },
            SwConfigParam {
                parameter_name: "PERF_DATABASE_VER".to_string(),
                value: "PDB-737-58".to_string(),
                default_value: "PDB-737-58".to_string(),
                editable: true,
            },
            SwConfigParam {
                parameter_name: "VNAV_IDLE_DESCENT_MODE".to_string(),
                value: "GEOMETRIC_PATH".to_string(),
                default_value: "GEOMETRIC_PATH".to_string(),
                editable: false,
            },
        ],
        last_load_date: "2026-03-01".to_string(),
        aircraft_reg: "EI-DCL".to_string(),
        approved_by: "AVIONICS-ENG-142".to_string(),
    };
    let encoded = encode_to_vec(&sw, cfg).expect("encode avionics sw config");
    let (decoded, _): (AvionicsSwConfig, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode avionics sw config");
    assert_eq!(sw, decoded);
}

#[test]
fn test_maintenance_forecast_roundtrip() {
    let cfg = config::standard();
    let forecast = MaintenanceForecast {
        aircraft_reg: "G-EZWC".to_string(),
        forecast_date: "2026-03-15".to_string(),
        current_flight_hours: 38420.5,
        current_cycles: 27810,
        daily_utilization_hrs: 10.2,
        daily_cycles: 6.5,
        upcoming_tasks: vec![
            ForecastTask {
                task_description: "Engine 1 borescope inspection (CFM SB 72-0987)".to_string(),
                due_at_hours: 39000.0,
                due_at_cycles: None,
                estimated_due_date: "2026-05-10".to_string(),
                source: "SB compliance".to_string(),
                can_package: true,
            },
            ForecastTask {
                task_description: "MLG retract actuator overhaul".to_string(),
                due_at_hours: 40000.0,
                due_at_cycles: Some(29000),
                estimated_due_date: "2026-08-20".to_string(),
                source: "Component TBO".to_string(),
                can_package: true,
            },
            ForecastTask {
                task_description: "AD 2026-04-12 wing spar inspection".to_string(),
                due_at_hours: 39500.0,
                due_at_cycles: Some(28500),
                estimated_due_date: "2026-06-28".to_string(),
                source: "AD compliance".to_string(),
                can_package: false,
            },
        ],
        next_heavy_check_type: "8Y / 2C Check".to_string(),
        next_heavy_check_due_date: "2027-06-15".to_string(),
        estimated_downtime_days: 35,
    };
    let encoded = encode_to_vec(&forecast, cfg).expect("encode maintenance forecast");
    let (decoded, _): (MaintenanceForecast, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode maintenance forecast");
    assert_eq!(forecast, decoded);
}

#[test]
fn test_multiple_gear_positions_roundtrip() {
    let cfg = config::standard();
    let gears = vec![
        LandingGearCycleTracker {
            gear_position: GearPosition::LeftMainGear,
            serial_number: "LMG-2022-0088".to_string(),
            total_cycles: 31200,
            cycles_since_overhaul: 11200,
            overhaul_limit_cycles: 20000,
            last_inspection_date: "2026-02-01".to_string(),
            tire_landings: 425,
            brake_wear_percent: 52.0,
            shimmy_damper_condition: "Serviceable".to_string(),
        },
        LandingGearCycleTracker {
            gear_position: GearPosition::RightMainGear,
            serial_number: "RMG-2022-0089".to_string(),
            total_cycles: 31200,
            cycles_since_overhaul: 11200,
            overhaul_limit_cycles: 20000,
            last_inspection_date: "2026-02-01".to_string(),
            tire_landings: 410,
            brake_wear_percent: 48.3,
            shimmy_damper_condition: "Serviceable".to_string(),
        },
        LandingGearCycleTracker {
            gear_position: GearPosition::CenterMainGear,
            serial_number: "CMG-2021-0031".to_string(),
            total_cycles: 15600,
            cycles_since_overhaul: 5600,
            overhaul_limit_cycles: 20000,
            last_inspection_date: "2026-01-15".to_string(),
            tire_landings: 280,
            brake_wear_percent: 22.1,
            shimmy_damper_condition: "Replaced - new unit installed".to_string(),
        },
    ];
    let encoded = encode_to_vec(&gears, cfg).expect("encode gear set");
    let (decoded, _): (Vec<LandingGearCycleTracker>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode gear set");
    assert_eq!(gears, decoded);
}

#[test]
fn test_engine_trend_batch_roundtrip() {
    let cfg = config::standard();
    let trends: Vec<EnginePerfTrend> = (0..5)
        .map(|i| EnginePerfTrend {
            engine_serial: "ESN-P2044B".to_string(),
            flight_id: 20260310000 + i as u64,
            timestamp_epoch: 1773350400 + (i as u64 * 3600),
            egt_celsius: 835.0 + (i as f64 * 2.1),
            n1_percent: 90.0 + (i as f64 * 0.3),
            n2_percent: 95.5 + (i as f64 * 0.2),
            fuel_flow_kg_per_hr: 2280.0 + (i as f64 * 15.0),
            oil_pressure_psi: 44.0 - (i as f64 * 0.1),
            oil_temp_celsius: 108.0 + (i as f64 * 0.5),
            vibration_ips: 0.9 + (i as f64 * 0.05),
            bleed_air_status: i % 2 == 0,
            altitude_ft: 33000 + (i as u32 * 1000),
            mach_number: 0.76 + (i as f64 * 0.01),
        })
        .collect();
    let encoded = encode_to_vec(&trends, cfg).expect("encode engine trend batch");
    let (decoded, _): (Vec<EnginePerfTrend>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode engine trend batch");
    assert_eq!(trends, decoded);
}

#[test]
fn test_complete_mro_package_roundtrip() {
    let cfg = config::standard();

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct MroPackage {
        aircraft_reg: String,
        check_type: String,
        logbook_entries: Vec<AircraftLogbookEntry>,
        active_ads: Vec<AirworthinessDirective>,
        mel_deferrals: Vec<MelDeferral>,
        work_orders: Vec<WorkOrder>,
        parts_consumed: Vec<PartsInventoryItem>,
    }

    let pkg = MroPackage {
        aircraft_reg: "SE-REX".to_string(),
        check_type: "A-Check".to_string(),
        logbook_entries: vec![AircraftLogbookEntry {
            entry_id: 90001,
            aircraft_reg: "SE-REX".to_string(),
            date_utc: "2026-03-14T06:00:00Z".to_string(),
            flight_hours: 18200.0,
            cycles: 12400,
            discrepancy: Some("Pack 1 valve cycling intermittently".to_string()),
            corrective_action: Some("Replaced pack valve, ops check good".to_string()),
            mechanic_license: "SE-LIC-44201".to_string(),
            inspector_license: Some("SE-IA-10032".to_string()),
            ata_chapter: 21,
        }],
        active_ads: vec![AirworthinessDirective {
            ad_number: "EASA-2025-0312".to_string(),
            title: "Fuel quantity indication system software update".to_string(),
            effective_date: "2025-12-15".to_string(),
            applicability: vec!["A320-214 MSN 5000-6000".to_string()],
            compliance_method: "Software load per Airbus SB A320-28-1412".to_string(),
            recurring: false,
            interval_hours: None,
            interval_cycles: None,
            interval_calendar_days: None,
            terminating_action: None,
            status: AdStatus::Complied,
        }],
        mel_deferrals: vec![],
        work_orders: vec![WorkOrder {
            wo_number: "WO-SE-2026-A001".to_string(),
            aircraft_reg: "SE-REX".to_string(),
            work_type: WorkType::Scheduled,
            priority: WoPriority::Routine,
            description: "A-Check package".to_string(),
            task_cards: vec![TaskCard {
                task_number: "12-00-00-001".to_string(),
                zone: "Zone 100".to_string(),
                skill_type: "Airframe".to_string(),
                estimated_hours: 2.0,
                completed: true,
            }],
            estimated_manhours: 120.0,
            actual_manhours: Some(115.5),
            status: WoStatus::Closed,
            open_date: "2026-03-12".to_string(),
            due_date: "2026-03-15".to_string(),
            close_date: Some("2026-03-14".to_string()),
        }],
        parts_consumed: vec![PartsInventoryItem {
            part_number: "AV16B2247".to_string(),
            description: "Pack valve, air conditioning".to_string(),
            category: PartCategory::Rotable,
            quantity_on_hand: 2,
            minimum_stock: 1,
            unit_price_cents: 4200000,
            shelf_life_days: None,
            certification: "EASA Form 1".to_string(),
            warehouse_location: "MRO-MAIN-A3-07".to_string(),
            last_receipt_date: "2025-11-10".to_string(),
            vendor_codes: vec!["V-LIEBHERR".to_string()],
        }],
    };

    let encoded = encode_to_vec(&pkg, cfg).expect("encode MRO package");
    let (decoded, _): (MroPackage, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode MRO package");
    assert_eq!(pkg, decoded);
}
