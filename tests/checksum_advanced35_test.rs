//! Advanced checksum tests for OxiCode — exactly 22 top-level #[test] functions.
//! Theme: Aviation MRO (Maintenance, Repair, Overhaul) and airworthiness management.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced35_test

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
// Test 1: Aircraft component life-tracking (cycles and flight hours)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ComponentLifeRecord {
    part_number: String,
    serial_number: String,
    total_flight_hours: f64,
    total_cycles: u64,
    hours_since_overhaul: f64,
    cycles_since_overhaul: u64,
    life_limit_hours: Option<f64>,
    life_limit_cycles: Option<u64>,
    is_life_limited: bool,
}

#[test]
fn test_component_life_tracking() {
    let record = ComponentLifeRecord {
        part_number: "PN-7842-CFM56".into(),
        serial_number: "ESN-508217".into(),
        total_flight_hours: 24_350.7,
        total_cycles: 18_422,
        hours_since_overhaul: 6_120.3,
        cycles_since_overhaul: 4_580,
        life_limit_hours: Some(30_000.0),
        life_limit_cycles: Some(25_000),
        is_life_limited: true,
    };
    let bytes = encode_with_checksum(&record).expect("encode component life record");
    let (decoded, _): (ComponentLifeRecord, _) =
        decode_with_checksum(&bytes).expect("decode component life record");
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// Test 2: MEL (Minimum Equipment List) deferral
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum MelCategory {
    CategoryA,
    CategoryB,
    CategoryC,
    CategoryD,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MelDeferral {
    deferral_number: String,
    ata_chapter: u8,
    item_description: String,
    category: MelCategory,
    max_deferral_days: u16,
    days_deferred: u16,
    operational_restrictions: Vec<String>,
    aircraft_registration: String,
    is_active: bool,
}

#[test]
fn test_mel_deferral() {
    let deferral = MelDeferral {
        deferral_number: "MEL-2026-0347".into(),
        ata_chapter: 34,
        item_description: "Weather Radar #2 inoperative".into(),
        category: MelCategory::CategoryC,
        max_deferral_days: 10,
        days_deferred: 3,
        operational_restrictions: vec![
            "Avoid areas of known thunderstorms".into(),
            "Dispatch with operable weather radar #1 only".into(),
        ],
        aircraft_registration: "JA8089".into(),
        is_active: true,
    };
    let bytes = encode_with_checksum(&deferral).expect("encode MEL deferral");
    let (decoded, _): (MelDeferral, _) = decode_with_checksum(&bytes).expect("decode MEL deferral");
    assert_eq!(decoded, deferral);
}

// ---------------------------------------------------------------------------
// Test 3: Engine borescope inspection record
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum BorescopeInspectionResult {
    Serviceable,
    MonitorNextInspection,
    RepairRequired,
    EngineRemovalRequired,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BorescopeRecord {
    engine_serial: String,
    engine_position: u8,
    inspection_type: String,
    hot_section_hours: f64,
    hpt_blade_condition: String,
    combustor_condition: String,
    result: BorescopeInspectionResult,
    next_inspection_interval_hours: u32,
    images_captured: u16,
}

#[test]
fn test_borescope_inspection() {
    let record = BorescopeRecord {
        engine_serial: "V2527-A5-ESN-30841".into(),
        engine_position: 1,
        inspection_type: "Hot Section Borescope".into(),
        hot_section_hours: 8_450.2,
        hpt_blade_condition: "Minor TBC spallation on blades 4, 17".into(),
        combustor_condition: "Acceptable wear pattern, no cracking".into(),
        result: BorescopeInspectionResult::MonitorNextInspection,
        next_inspection_interval_hours: 1_500,
        images_captured: 128,
    };
    let bytes = encode_with_checksum(&record).expect("encode borescope record");
    let (decoded, _): (BorescopeRecord, _) =
        decode_with_checksum(&bytes).expect("decode borescope record");
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// Test 4: Landing gear overhaul data
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct LandingGearOverhaul {
    gear_position: String,
    serial_number: String,
    overhaul_facility: String,
    total_landings: u64,
    landings_since_overhaul: u64,
    overhaul_interval_landings: u64,
    chrome_plating_thickness_microns: u32,
    bushing_wear_mm: u16,
    torque_link_play_mm: u16,
    hydraulic_seal_replaced: bool,
    axle_ndt_passed: bool,
}

#[test]
fn test_landing_gear_overhaul() {
    let overhaul = LandingGearOverhaul {
        gear_position: "NLG (Nose Landing Gear)".into(),
        serial_number: "NLG-SN-44291".into(),
        overhaul_facility: "Lufthansa Technik Hamburg".into(),
        total_landings: 32_150,
        landings_since_overhaul: 0,
        overhaul_interval_landings: 20_000,
        chrome_plating_thickness_microns: 250,
        bushing_wear_mm: 0,
        torque_link_play_mm: 1,
        hydraulic_seal_replaced: true,
        axle_ndt_passed: true,
    };
    let bytes = encode_with_checksum(&overhaul).expect("encode landing gear overhaul");
    let (decoded, _): (LandingGearOverhaul, _) =
        decode_with_checksum(&bytes).expect("decode landing gear overhaul");
    assert_eq!(decoded, overhaul);
}

// ---------------------------------------------------------------------------
// Test 5: Avionics LRU (Line Replaceable Unit) swap record
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct LruSwapRecord {
    lru_name: String,
    removed_part_number: String,
    removed_serial_number: String,
    installed_part_number: String,
    installed_serial_number: String,
    ata_chapter: u8,
    ata_section: u8,
    reason_for_removal: String,
    aircraft_registration: String,
    flight_hours_at_removal: f64,
    bit_test_passed: bool,
}

#[test]
fn test_avionics_lru_swap() {
    let swap = LruSwapRecord {
        lru_name: "FMGC (Flight Management Guidance Computer)".into(),
        removed_part_number: "A320-FMGC-V3".into(),
        removed_serial_number: "FMGC-SN-09821".into(),
        installed_part_number: "A320-FMGC-V3".into(),
        installed_serial_number: "FMGC-SN-12044".into(),
        ata_chapter: 22,
        ata_section: 11,
        reason_for_removal: "FMS position drift beyond tolerance".into(),
        aircraft_registration: "D-AIUA".into(),
        flight_hours_at_removal: 41_200.5,
        bit_test_passed: true,
    };
    let bytes = encode_with_checksum(&swap).expect("encode LRU swap record");
    let (decoded, _): (LruSwapRecord, _) =
        decode_with_checksum(&bytes).expect("decode LRU swap record");
    assert_eq!(decoded, swap);
}

// ---------------------------------------------------------------------------
// Test 6: Airworthiness Directive (AD) compliance
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum AdComplianceStatus {
    NotApplicable,
    Open,
    Complied,
    Recurring,
    Terminated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AirworthinessDirective {
    ad_number: String,
    issuing_authority: String,
    affected_type_certificate: String,
    subject: String,
    compliance_status: AdComplianceStatus,
    compliance_hours: Option<f64>,
    compliance_cycles: Option<u64>,
    recurring_interval_hours: Option<f64>,
    is_emergency_ad: bool,
    superseded_by: Option<String>,
}

#[test]
fn test_airworthiness_directive_compliance() {
    let ad = AirworthinessDirective {
        ad_number: "EASA-2025-0412-E".into(),
        issuing_authority: "EASA".into(),
        affected_type_certificate: "A320-200 (TCDS A.064)".into(),
        subject: "Wing-to-fuselage frame 47.5 fatigue cracking inspection".into(),
        compliance_status: AdComplianceStatus::Recurring,
        compliance_hours: Some(38_500.0),
        compliance_cycles: Some(28_000),
        recurring_interval_hours: Some(6_000.0),
        is_emergency_ad: true,
        superseded_by: None,
    };
    let bytes = encode_with_checksum(&ad).expect("encode AD compliance record");
    let (decoded, _): (AirworthinessDirective, _) =
        decode_with_checksum(&bytes).expect("decode AD compliance record");
    assert_eq!(decoded, ad);
}

// ---------------------------------------------------------------------------
// Test 7: Workorder task card
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum TaskCardStatus {
    Planned,
    InProgress,
    PendingInspection,
    Completed,
    Deferred,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WorkorderTaskCard {
    workorder_number: String,
    task_card_number: String,
    mpd_reference: String,
    ata_chapter: u8,
    description: String,
    status: TaskCardStatus,
    estimated_manhours: f64,
    actual_manhours: f64,
    zone: String,
    access_panels: Vec<String>,
    requires_ndt: bool,
    requires_duplicate_inspection: bool,
}

#[test]
fn test_workorder_task_card() {
    let card = WorkorderTaskCard {
        workorder_number: "WO-2026-C-00418".into(),
        task_card_number: "TC-53-21-001".into(),
        mpd_reference: "MPD 53-21-01-01".into(),
        ata_chapter: 53,
        description: "Fuselage skin lap joint inspection at STA 540-620".into(),
        status: TaskCardStatus::PendingInspection,
        estimated_manhours: 8.5,
        actual_manhours: 7.2,
        zone: "Zone 400 (Lower fuselage)".into(),
        access_panels: vec!["441AB".into(), "441CD".into(), "442AB".into()],
        requires_ndt: true,
        requires_duplicate_inspection: true,
    };
    let bytes = encode_with_checksum(&card).expect("encode workorder task card");
    let (decoded, _): (WorkorderTaskCard, _) =
        decode_with_checksum(&bytes).expect("decode workorder task card");
    assert_eq!(decoded, card);
}

// ---------------------------------------------------------------------------
// Test 8: Parts traceability (birth record)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PartBirthRecord {
    part_number: String,
    serial_number: String,
    batch_number: String,
    manufacturer: String,
    manufacturing_date_epoch_secs: u64,
    certification_basis: String,
    easa_form1_number: Option<String>,
    faa_8130_number: Option<String>,
    material_specification: String,
    heat_treatment_code: String,
    shelf_life_months: Option<u16>,
}

#[test]
fn test_parts_traceability_birth_record() {
    let record = PartBirthRecord {
        part_number: "HPT-BLD-CFM56-5B".into(),
        serial_number: "BLD-SN-220491".into(),
        batch_number: "BATCH-2025-Q3-0071".into(),
        manufacturer: "Safran Aircraft Engines".into(),
        manufacturing_date_epoch_secs: 1_756_000_000,
        certification_basis: "EASA Part 21 Subpart G POA".into(),
        easa_form1_number: Some("EASA-F1-2025-884512".into()),
        faa_8130_number: None,
        material_specification: "PWA 1484 Single Crystal Nickel Alloy".into(),
        heat_treatment_code: "HT-SX-1320C-4HR".into(),
        shelf_life_months: None,
    };
    let bytes = encode_with_checksum(&record).expect("encode part birth record");
    let (decoded, _): (PartBirthRecord, _) =
        decode_with_checksum(&bytes).expect("decode part birth record");
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// Test 9: NDT (Non-Destructive Testing) result
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum NdtMethod {
    EddyCurrent,
    Ultrasonic,
    MagneticParticle,
    FluorescentPenetrant,
    Radiographic,
    Thermographic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum NdtVerdict {
    NoIndication,
    AcceptableIndication,
    RejectableIndication,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NdtResult {
    method: NdtMethod,
    procedure_reference: String,
    inspector_certification_level: u8,
    component_description: String,
    indication_length_mm: Option<u32>,
    indication_depth_mm: Option<u32>,
    verdict: NdtVerdict,
    calibration_block_serial: String,
    equipment_serial: String,
}

#[test]
fn test_ndt_result() {
    let result = NdtResult {
        method: NdtMethod::EddyCurrent,
        procedure_reference: "NTM-EC-53-001 Rev 12".into(),
        inspector_certification_level: 2,
        component_description: "Wing lower skin panel at WS 240-280".into(),
        indication_length_mm: Some(4),
        indication_depth_mm: Some(1),
        verdict: NdtVerdict::AcceptableIndication,
        calibration_block_serial: "CAL-EC-0098".into(),
        equipment_serial: "NORTEC-600S-SN-4412".into(),
    };
    let bytes = encode_with_checksum(&result).expect("encode NDT result");
    let (decoded, _): (NdtResult, _) = decode_with_checksum(&bytes).expect("decode NDT result");
    assert_eq!(decoded, result);
}

// ---------------------------------------------------------------------------
// Test 10: Flight data recorder parameters
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct FdrParameterSnapshot {
    flight_number: String,
    timestamp_utc_epoch_ms: u64,
    altitude_ft: i32,
    airspeed_kts: u16,
    heading_deg: u16,
    pitch_deg: i16,
    roll_deg: i16,
    vertical_speed_fpm: i32,
    engine1_n1_pct_x10: u16,
    engine2_n1_pct_x10: u16,
    flap_position_deg: u8,
    gear_down: bool,
    autopilot_engaged: bool,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
}

#[test]
fn test_fdr_parameter_snapshot() {
    let snapshot = FdrParameterSnapshot {
        flight_number: "NH241".into(),
        timestamp_utc_epoch_ms: 1_773_000_000_000,
        altitude_ft: 35_000,
        airspeed_kts: 280,
        heading_deg: 73,
        pitch_deg: 25,
        roll_deg: -3,
        vertical_speed_fpm: 0,
        engine1_n1_pct_x10: 872,
        engine2_n1_pct_x10: 869,
        flap_position_deg: 0,
        gear_down: false,
        autopilot_engaged: true,
        latitude_microdeg: 35_680_000,
        longitude_microdeg: 139_770_000,
    };
    let bytes = encode_with_checksum(&snapshot).expect("encode FDR snapshot");
    let (decoded, _): (FdrParameterSnapshot, _) =
        decode_with_checksum(&bytes).expect("decode FDR snapshot");
    assert_eq!(decoded, snapshot);
}

// ---------------------------------------------------------------------------
// Test 11: ETOPS certification requirements
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct EtopsCertification {
    aircraft_type: String,
    operator: String,
    etops_diversion_time_minutes: u16,
    engine_type_certificate: String,
    ifsd_rate_target: f64,
    ifsd_rate_actual: f64,
    maintenance_program_approved: bool,
    required_systems: Vec<String>,
    alternate_airports_count: u8,
    fuel_reserve_policy_minutes: u16,
}

#[test]
fn test_etops_certification() {
    let cert = EtopsCertification {
        aircraft_type: "B787-9".into(),
        operator: "ANA (All Nippon Airways)".into(),
        etops_diversion_time_minutes: 180,
        engine_type_certificate: "GEnx-1B74/75".into(),
        ifsd_rate_target: 0.02,
        ifsd_rate_actual: 0.005,
        maintenance_program_approved: true,
        required_systems: vec![
            "Dual cargo fire suppression".into(),
            "APU auto-start capability".into(),
            "Dual HF communication".into(),
            "Fuel crossfeed system".into(),
        ],
        alternate_airports_count: 6,
        fuel_reserve_policy_minutes: 15,
    };
    let bytes = encode_with_checksum(&cert).expect("encode ETOPS certification");
    let (decoded, _): (EtopsCertification, _) =
        decode_with_checksum(&bytes).expect("decode ETOPS certification");
    assert_eq!(decoded, cert);
}

// ---------------------------------------------------------------------------
// Test 12: CPCP (Corrosion Prevention and Control Program)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum CorrosionLevel {
    Level1Clean,
    Level2Light,
    Level3Moderate,
    Level4Severe,
    Level5VeryHeavy,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CpcpInspection {
    task_reference: String,
    zone: String,
    structural_element: String,
    corrosion_level: CorrosionLevel,
    area_affected_sq_cm: u32,
    depth_removed_mm: u16,
    allowable_damage_limit_mm: u16,
    treatment_applied: String,
    primer_applied: bool,
    next_interval_months: u16,
}

#[test]
fn test_cpcp_inspection() {
    let inspection = CpcpInspection {
        task_reference: "CPCP-53-42-001".into(),
        zone: "Zone 142 (Belly fairing area)".into(),
        structural_element: "Frame STA 820 lower flange".into(),
        corrosion_level: CorrosionLevel::Level2Light,
        area_affected_sq_cm: 45,
        depth_removed_mm: 2,
        allowable_damage_limit_mm: 8,
        treatment_applied: "Alodine 1201 conversion coating".into(),
        primer_applied: true,
        next_interval_months: 48,
    };
    let bytes = encode_with_checksum(&inspection).expect("encode CPCP inspection");
    let (decoded, _): (CpcpInspection, _) =
        decode_with_checksum(&bytes).expect("decode CPCP inspection");
    assert_eq!(decoded, inspection);
}

// ---------------------------------------------------------------------------
// Test 13: Cabin interior modification (STC)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct SupplementalTypeCertificate {
    stc_number: String,
    title: String,
    applicable_aircraft: Vec<String>,
    holder: String,
    modification_description: String,
    weight_change_kg: i32,
    cg_impact_percent_mac: f64,
    ifs_drawing_count: u16,
    embodiment_manhours: f64,
}

#[test]
fn test_supplemental_type_certificate() {
    let stc = SupplementalTypeCertificate {
        stc_number: "EASA.STC.10082394".into(),
        title: "Premium economy seat installation B777-300ER".into(),
        applicable_aircraft: vec!["B777-300ER".into(), "B777-300".into()],
        holder: "JAL Engineering Co.".into(),
        modification_description:
            "Installation of 40 premium economy seats rows 18-27, LOPA revision".into(),
        weight_change_kg: 320,
        cg_impact_percent_mac: -0.3,
        ifs_drawing_count: 47,
        embodiment_manhours: 1_200.0,
    };
    let bytes = encode_with_checksum(&stc).expect("encode STC");
    let (decoded, _): (SupplementalTypeCertificate, _) =
        decode_with_checksum(&bytes).expect("decode STC");
    assert_eq!(decoded, stc);
}

// ---------------------------------------------------------------------------
// Test 14: Hydraulic system fluid analysis
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct HydraulicFluidAnalysis {
    system_id: String,
    fluid_type: String,
    sample_date_epoch_secs: u64,
    contamination_class_iso4406: String,
    water_content_ppm: u32,
    acidity_mg_koh_per_gram: u16,
    particulate_count_per_ml: u32,
    copper_ppm: u16,
    iron_ppm: u16,
    serviceable: bool,
    next_sample_flight_hours: f64,
}

#[test]
fn test_hydraulic_fluid_analysis() {
    let analysis = HydraulicFluidAnalysis {
        system_id: "HYD-SYS-GREEN (3000 PSI)".into(),
        fluid_type: "Skydrol LD-4".into(),
        sample_date_epoch_secs: 1_773_100_000,
        contamination_class_iso4406: "18/16/13".into(),
        water_content_ppm: 420,
        acidity_mg_koh_per_gram: 15,
        particulate_count_per_ml: 8_500,
        copper_ppm: 3,
        iron_ppm: 12,
        serviceable: true,
        next_sample_flight_hours: 500.0,
    };
    let bytes = encode_with_checksum(&analysis).expect("encode hydraulic fluid analysis");
    let (decoded, _): (HydraulicFluidAnalysis, _) =
        decode_with_checksum(&bytes).expect("decode hydraulic fluid analysis");
    assert_eq!(decoded, analysis);
}

// ---------------------------------------------------------------------------
// Test 15: Tire and wheel assembly record
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct TireWheelAssembly {
    position: String,
    tire_part_number: String,
    tire_serial: String,
    tire_retreads_count: u8,
    tire_landings_since_install: u32,
    remaining_tread_depth_mm: u16,
    wheel_serial: String,
    wheel_overhauls_count: u8,
    brake_assembly_serial: String,
    brake_wear_indicator_remaining_pct: u8,
    inflation_pressure_psi: u16,
    nitrogen_filled: bool,
}

#[test]
fn test_tire_wheel_assembly() {
    let assembly = TireWheelAssembly {
        position: "MLG-L2 (Main Landing Gear Left #2)".into(),
        tire_part_number: "H40x14.5-19 Radial".into(),
        tire_serial: "MICH-SN-90127".into(),
        tire_retreads_count: 2,
        tire_landings_since_install: 180,
        remaining_tread_depth_mm: 7,
        wheel_serial: "WHL-SN-55890".into(),
        wheel_overhauls_count: 3,
        brake_assembly_serial: "BRK-C-SN-14820".into(),
        brake_wear_indicator_remaining_pct: 62,
        inflation_pressure_psi: 210,
        nitrogen_filled: true,
    };
    let bytes = encode_with_checksum(&assembly).expect("encode tire wheel assembly");
    let (decoded, _): (TireWheelAssembly, _) =
        decode_with_checksum(&bytes).expect("decode tire wheel assembly");
    assert_eq!(decoded, assembly);
}

// ---------------------------------------------------------------------------
// Test 16: Structural repair record (SRM-based)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum RepairClassification {
    MinorDamage,
    AllowableDamage,
    StandardRepair,
    MajorRepair,
    DamageToleranceRepair,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StructuralRepairRecord {
    repair_number: String,
    srm_chapter: String,
    location_description: String,
    damage_length_mm: u32,
    damage_width_mm: u32,
    damage_depth_mm: u16,
    classification: RepairClassification,
    doubler_installed: bool,
    fastener_count: u16,
    sealant_applied: String,
    design_approval_required: bool,
    repeat_inspection_interval_hours: Option<f64>,
}

#[test]
fn test_structural_repair_record() {
    let repair = StructuralRepairRecord {
        repair_number: "SR-2026-0089".into(),
        srm_chapter: "SRM 53-40-11".into(),
        location_description: "Fuselage skin panel S-14R between STA 740-760".into(),
        damage_length_mm: 120,
        damage_width_mm: 45,
        damage_depth_mm: 3,
        classification: RepairClassification::StandardRepair,
        doubler_installed: true,
        fastener_count: 48,
        sealant_applied: "PR-1776 Class B".into(),
        design_approval_required: false,
        repeat_inspection_interval_hours: Some(12_000.0),
    };
    let bytes = encode_with_checksum(&repair).expect("encode structural repair");
    let (decoded, _): (StructuralRepairRecord, _) =
        decode_with_checksum(&bytes).expect("decode structural repair");
    assert_eq!(decoded, repair);
}

// ---------------------------------------------------------------------------
// Test 17: APU (Auxiliary Power Unit) health monitoring
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ApuHealthMonitoring {
    apu_model: String,
    apu_serial: String,
    total_apu_hours: f64,
    total_apu_cycles: u64,
    egt_margin_deg_c: i32,
    oil_consumption_liters_per_hour: f64,
    bleed_air_performance_pct: u8,
    starter_duty_cycle_attempts: u32,
    inlet_plenum_condition: String,
    trend_monitoring_alert: bool,
    recommended_action: String,
}

#[test]
fn test_apu_health_monitoring() {
    let apu = ApuHealthMonitoring {
        apu_model: "APS3200".into(),
        apu_serial: "APU-SN-18437".into(),
        total_apu_hours: 19_800.4,
        total_apu_cycles: 42_100,
        egt_margin_deg_c: 35,
        oil_consumption_liters_per_hour: 0.12,
        bleed_air_performance_pct: 94,
        starter_duty_cycle_attempts: 3,
        inlet_plenum_condition: "Light FOD marks, serviceable".into(),
        trend_monitoring_alert: false,
        recommended_action: "Continue monitoring, next trend review at 20,000 hrs".into(),
    };
    let bytes = encode_with_checksum(&apu).expect("encode APU health monitoring");
    let (decoded, _): (ApuHealthMonitoring, _) =
        decode_with_checksum(&bytes).expect("decode APU health monitoring");
    assert_eq!(decoded, apu);
}

// ---------------------------------------------------------------------------
// Test 18: CAMO (Continuing Airworthiness Management Organization) status
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct CamoAircraftStatus {
    registration: String,
    aircraft_type: String,
    msn: String,
    total_flight_hours: f64,
    total_cycles: u64,
    next_a_check_hours_remaining: f64,
    next_c_check_hours_remaining: f64,
    next_structural_check_months_remaining: u16,
    open_ad_count: u16,
    open_sb_count: u16,
    open_mel_count: u8,
    airworthiness_review_certificate_valid_until_epoch: u64,
    insurance_valid: bool,
}

#[test]
fn test_camo_aircraft_status() {
    let status = CamoAircraftStatus {
        registration: "JA873A".into(),
        aircraft_type: "B787-9".into(),
        msn: "MSN-34527".into(),
        total_flight_hours: 28_450.6,
        total_cycles: 6_420,
        next_a_check_hours_remaining: 340.0,
        next_c_check_hours_remaining: 4_200.0,
        next_structural_check_months_remaining: 36,
        open_ad_count: 2,
        open_sb_count: 14,
        open_mel_count: 1,
        airworthiness_review_certificate_valid_until_epoch: 1_788_000_000,
        insurance_valid: true,
    };
    let bytes = encode_with_checksum(&status).expect("encode CAMO aircraft status");
    let (decoded, _): (CamoAircraftStatus, _) =
        decode_with_checksum(&bytes).expect("decode CAMO aircraft status");
    assert_eq!(decoded, status);
}

// ---------------------------------------------------------------------------
// Test 19: Fuel tank inspection (aging aircraft)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum FuelTankSealantCondition {
    IntactNoDefects,
    MinorCracking,
    Peeling,
    RequiresReseal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FuelTankInspection {
    tank_designation: String,
    capacity_liters: u32,
    inspection_type: String,
    sealant_condition: FuelTankSealantCondition,
    leak_rate_drops_per_minute: u8,
    wiring_condition_satisfactory: bool,
    bonding_resistance_milliohms: u16,
    debris_found: bool,
    debris_description: Option<String>,
    next_inspection_interval_months: u16,
}

#[test]
fn test_fuel_tank_inspection() {
    let inspection = FuelTankInspection {
        tank_designation: "Center Wing Tank (CWT)".into(),
        capacity_liters: 28_500,
        inspection_type: "CDCCL Fuel Tank Safety Inspection".into(),
        sealant_condition: FuelTankSealantCondition::MinorCracking,
        leak_rate_drops_per_minute: 0,
        wiring_condition_satisfactory: true,
        bonding_resistance_milliohms: 3,
        debris_found: true,
        debris_description: Some("Minor sealant chips on tank floor, removed".into()),
        next_inspection_interval_months: 72,
    };
    let bytes = encode_with_checksum(&inspection).expect("encode fuel tank inspection");
    let (decoded, _): (FuelTankInspection, _) =
        decode_with_checksum(&bytes).expect("decode fuel tank inspection");
    assert_eq!(decoded, inspection);
}

// ---------------------------------------------------------------------------
// Test 20: Engine performance restoration event
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum ShopVisitScope {
    PerformanceRestoration,
    HotSectionInspection,
    FullOverhaul,
    LimitedOverhaul,
    QuickTurn,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EngineShopVisit {
    engine_model: String,
    engine_serial: String,
    shop_facility: String,
    visit_scope: ShopVisitScope,
    tsn_at_induction_hours: f64,
    csn_at_induction: u64,
    tso_at_induction_hours: f64,
    cso_at_induction: u64,
    egt_margin_restored_deg_c: u32,
    modules_replaced: Vec<String>,
    llp_remaining_cycles_min: u64,
    test_cell_run_completed: bool,
    green_time_hours: f64,
}

#[test]
fn test_engine_shop_visit() {
    let visit = EngineShopVisit {
        engine_model: "CF34-10E7".into(),
        engine_serial: "ESN-197422".into(),
        shop_facility: "StandardAero San Antonio".into(),
        visit_scope: ShopVisitScope::PerformanceRestoration,
        tsn_at_induction_hours: 22_100.0,
        csn_at_induction: 16_800,
        tso_at_induction_hours: 11_050.0,
        cso_at_induction: 8_400,
        egt_margin_restored_deg_c: 55,
        modules_replaced: vec![
            "HPT module (Stage 1 blades)".into(),
            "Combustor liners".into(),
        ],
        llp_remaining_cycles_min: 6_200,
        test_cell_run_completed: true,
        green_time_hours: 2.5,
    };
    let bytes = encode_with_checksum(&visit).expect("encode engine shop visit");
    let (decoded, _): (EngineShopVisit, _) =
        decode_with_checksum(&bytes).expect("decode engine shop visit");
    assert_eq!(decoded, visit);
}

// ---------------------------------------------------------------------------
// Test 21: EWIS (Electrical Wiring Interconnection System) inspection
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum WireCondition {
    Serviceable,
    Chafed,
    Cracked,
    ArcDamage,
    ContaminatedFluid,
    RequiresReplacement,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EwisInspection {
    zone: String,
    wire_bundle_id: String,
    wire_gauge_awg: u8,
    wire_type: String,
    condition: WireCondition,
    clamp_spacing_adequate: bool,
    separation_from_hydraulic_adequate: bool,
    separation_from_fuel_adequate: bool,
    connector_condition: String,
    shielding_integrity_verified: bool,
    findings_count: u8,
    corrective_action: Option<String>,
}

#[test]
fn test_ewis_inspection() {
    let inspection = EwisInspection {
        zone: "Zone 115 (Wheel well area)".into(),
        wire_bundle_id: "WB-32L-115-004".into(),
        wire_gauge_awg: 20,
        wire_type: "BMS 13-60 Type IX PTFE/Polyimide".into(),
        condition: WireCondition::Chafed,
        clamp_spacing_adequate: false,
        separation_from_hydraulic_adequate: true,
        separation_from_fuel_adequate: true,
        connector_condition: "Pins clean, no corrosion, proper seating verified".into(),
        shielding_integrity_verified: true,
        findings_count: 2,
        corrective_action: Some(
            "Sleeve installed at chafe point, clamp repositioned per WDM 20-10-11".into(),
        ),
    };
    let bytes = encode_with_checksum(&inspection).expect("encode EWIS inspection");
    let (decoded, _): (EwisInspection, _) =
        decode_with_checksum(&bytes).expect("decode EWIS inspection");
    assert_eq!(decoded, inspection);
}

// ---------------------------------------------------------------------------
// Test 22: RVSM (Reduced Vertical Separation Minimum) compliance
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct RvsmCompliance {
    aircraft_registration: String,
    altimeter1_serial: String,
    altimeter2_serial: String,
    static_source_error_ft: i16,
    altimetry_system_error_ft: i16,
    altitude_keeping_capability_ft: u16,
    autopilot_altitude_hold_verified: bool,
    altitude_alerting_system_verified: bool,
    transponder_mode_c_verified: bool,
    height_monitoring_group_correlation_epoch: u64,
    hmg_deviation_ft: i16,
    rvsm_approval_valid: bool,
    next_rvsm_check_months: u8,
}

#[test]
fn test_rvsm_compliance() {
    let compliance = RvsmCompliance {
        aircraft_registration: "JA742J".into(),
        altimeter1_serial: "ALT-SN-88210".into(),
        altimeter2_serial: "ALT-SN-88211".into(),
        static_source_error_ft: -12,
        altimetry_system_error_ft: 25,
        altitude_keeping_capability_ft: 40,
        autopilot_altitude_hold_verified: true,
        altitude_alerting_system_verified: true,
        transponder_mode_c_verified: true,
        height_monitoring_group_correlation_epoch: 1_770_000_000,
        hmg_deviation_ft: -18,
        rvsm_approval_valid: true,
        next_rvsm_check_months: 24,
    };
    let bytes = encode_with_checksum(&compliance).expect("encode RVSM compliance");
    let (decoded, _): (RvsmCompliance, _) =
        decode_with_checksum(&bytes).expect("decode RVSM compliance");
    assert_eq!(decoded, compliance);
}
