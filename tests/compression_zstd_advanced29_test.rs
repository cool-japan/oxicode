//! Advanced Zstd compression tests for OxiCode — Pest Control & Integrated
//! Pest Management (IPM) domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world pest control operations: inspection reports, pest
//! identification, treatment application records, bait station monitoring,
//! fumigation protocols, wildlife exclusion, bed bug heat treatment, termite
//! colony tracking, regulatory licenses, service agreements, rodent activity
//! maps, and vegetation management schedules.

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
// Domain types — Enumerations
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestCategory {
    Insect,
    Arachnid,
    Rodent,
    Bird,
    Wildlife,
    Fungal,
    Weed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SeverityLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TreatmentMethod {
    ChemicalSpray,
    ChemicalBait,
    BiologicalPredator,
    BiologicalPathogen,
    MechanicalTrap,
    MechanicalExclusion,
    HeatTreatment,
    Fumigation,
    CulturalPractice,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BaitStationStatus {
    Active,
    Inactive,
    NeedsRefill,
    Damaged,
    Removed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FumigantType {
    SulfurylFluoride,
    MethylBromide,
    PhosphineGas,
    Chloropicrin,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExclusionMaterial {
    SteelWool,
    CopperMesh,
    ExpandingFoam,
    HardwareCloth,
    MetalFlashing,
    WeatherStripping,
    ConcreteRepair,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LicenseType {
    GeneralPest,
    TermiteControl,
    FumigationCertified,
    WildlifeManagement,
    VegetationManagement,
    StructuralPestControl,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RodentSpecies {
    NorwayRat,
    RoofRat,
    HouseMouse,
    DeerMouse,
    Vole,
    Squirrel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TermiteCasteType {
    Worker,
    Soldier,
    Reproductive,
    Nymph,
    King,
    Queen,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VegetationTreatmentType {
    PreEmergentHerbicide,
    PostEmergentHerbicide,
    Mowing,
    ManualRemoval,
    MulchApplication,
    ControlledBurn,
}

// ---------------------------------------------------------------------------
// Domain types — Structs
// ---------------------------------------------------------------------------

/// A full property inspection report.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionReport {
    report_id: u64,
    inspector_license_id: u32,
    property_address_hash: u64,
    inspection_date_epoch_sec: u64,
    total_area_sqft: u32,
    rooms_inspected: u16,
    findings: Vec<PestFinding>,
    recommendations: Vec<String>,
    follow_up_days: u16,
}

/// A single pest finding within an inspection.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PestFinding {
    pest_species: String,
    category: PestCategory,
    severity: SeverityLevel,
    evidence_count: u32,
    location_description: String,
    photo_hash: u64,
}

/// Treatment application record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentRecord {
    record_id: u64,
    report_id: u64,
    technician_id: u32,
    method: TreatmentMethod,
    product_name: String,
    epa_registration: String,
    active_ingredient: String,
    concentration_ppm: u32,
    volume_applied_ml: u32,
    application_date_epoch_sec: u64,
    target_pest: String,
    wind_speed_kph_x10: u16,
    temperature_c_x10: i16,
    humidity_pct: u8,
}

/// Bait station monitoring entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BaitStationEntry {
    station_id: u32,
    latitude_microdeg: i32,
    longitude_microdeg: i32,
    status: BaitStationStatus,
    bait_type: String,
    bait_remaining_grams: u16,
    consumption_grams: u16,
    last_checked_epoch_sec: u64,
    rodent_activity_score: u8,
    notes: String,
}

/// Fumigation protocol.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FumigationProtocol {
    protocol_id: u64,
    structure_volume_cuft: u32,
    fumigant: FumigantType,
    dosage_oz_per_1000cuft: u16,
    exposure_hours: u16,
    clearance_readings: Vec<u32>,
    tarp_seal_integrity_pct: u8,
    temperature_c_x10: i16,
    warning_placards_count: u8,
    aeration_start_epoch_sec: u64,
    aeration_end_epoch_sec: u64,
}

/// Wildlife exclusion work order.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WildlifeExclusionWork {
    work_order_id: u64,
    target_species: String,
    entry_points: Vec<ExclusionPoint>,
    total_materials_cost_cents: u32,
    labor_hours_x10: u16,
    warranty_months: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExclusionPoint {
    location: String,
    opening_size_mm: u16,
    material_used: ExclusionMaterial,
    sealed: bool,
}

/// Bed bug heat treatment parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BedBugHeatTreatment {
    treatment_id: u64,
    room_sqft: u32,
    target_temp_f_x10: u16,
    hold_duration_minutes: u16,
    sensor_readings: Vec<HeatSensorReading>,
    prep_checklist_complete: bool,
    heater_count: u8,
    fan_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeatSensorReading {
    sensor_id: u8,
    timestamp_offset_sec: u32,
    temperature_f_x10: u16,
}

/// Termite colony tracking record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TermiteColonyRecord {
    colony_id: u64,
    species: String,
    estimated_population: u64,
    caste_counts: Vec<(TermiteCasteType, u32)>,
    monitoring_stations: Vec<u32>,
    bait_matrix_consumption_g: u32,
    first_detected_epoch_sec: u64,
    last_activity_epoch_sec: u64,
    colony_eliminated: bool,
}

/// Regulatory license record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegulatoryLicense {
    license_number: String,
    holder_name: String,
    license_type: LicenseType,
    state_code: String,
    issued_epoch_sec: u64,
    expires_epoch_sec: u64,
    continuing_education_hours: u16,
    violations_count: u8,
    active: bool,
}

/// Customer service agreement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ServiceAgreement {
    agreement_id: u64,
    customer_id: u64,
    property_address_hash: u64,
    service_types: Vec<String>,
    monthly_fee_cents: u32,
    start_epoch_sec: u64,
    end_epoch_sec: u64,
    visit_frequency_days: u16,
    covered_pests: Vec<String>,
    exclusions: Vec<String>,
    auto_renew: bool,
}

/// A rodent activity map cell.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RodentActivityCell {
    grid_x: u16,
    grid_y: u16,
    species_detected: Vec<RodentSpecies>,
    droppings_count: u32,
    gnaw_marks: bool,
    burrow_entries: u8,
    trail_camera_hits: u16,
}

/// Rodent activity map covering a property.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RodentActivityMap {
    map_id: u64,
    property_id: u64,
    grid_cols: u16,
    grid_rows: u16,
    cell_size_ft_x10: u16,
    survey_epoch_sec: u64,
    cells: Vec<RodentActivityCell>,
}

/// Vegetation management schedule entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VegetationScheduleEntry {
    entry_id: u64,
    zone_name: String,
    area_sqft: u32,
    treatment: VegetationTreatmentType,
    product_name: String,
    application_rate_oz_per_acre_x10: u16,
    scheduled_epoch_sec: u64,
    completed: bool,
    re_treatment_interval_days: u16,
}

/// An IPM strategy plan that brings together multiple control tactics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IpmStrategyPlan {
    plan_id: u64,
    property_id: u64,
    target_pest: String,
    category: PestCategory,
    action_thresholds: Vec<(String, u32)>,
    monitoring_interval_days: u16,
    cultural_controls: Vec<String>,
    biological_controls: Vec<String>,
    chemical_controls: Vec<String>,
    mechanical_controls: Vec<String>,
    evaluation_date_epoch_sec: u64,
}

/// A pest control equipment calibration record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentCalibration {
    equipment_id: u32,
    equipment_type: String,
    serial_number: String,
    calibration_date_epoch_sec: u64,
    next_due_epoch_sec: u64,
    flow_rate_ml_per_min_x10: u32,
    pressure_psi_x10: u16,
    nozzle_pattern: String,
    passed: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_pest_finding(idx: u32) -> PestFinding {
    let (species, cat, sev) = match idx % 5 {
        0 => (
            "German Cockroach",
            PestCategory::Insect,
            SeverityLevel::High,
        ),
        1 => (
            "Brown Recluse Spider",
            PestCategory::Arachnid,
            SeverityLevel::Moderate,
        ),
        2 => ("Norway Rat", PestCategory::Rodent, SeverityLevel::Critical),
        3 => ("House Sparrow", PestCategory::Bird, SeverityLevel::Low),
        _ => (
            "Subterranean Termite",
            PestCategory::Insect,
            SeverityLevel::Critical,
        ),
    };
    PestFinding {
        pest_species: species.to_string(),
        category: cat,
        severity: sev,
        evidence_count: 10 + idx * 3,
        location_description: format!("Zone {}, Room {}", idx / 4 + 1, idx % 4 + 1),
        photo_hash: 0xDEAD_BEEF_0000_0000 + idx as u64,
    }
}

fn make_inspection_report(id: u64) -> InspectionReport {
    InspectionReport {
        report_id: id,
        inspector_license_id: 50000 + id as u32,
        property_address_hash: 0xABCD_1234_0000_0000 + id,
        inspection_date_epoch_sec: 1_700_000_000 + id * 86400,
        total_area_sqft: 2400 + (id * 100) as u32,
        rooms_inspected: 8 + (id % 5) as u16,
        findings: (0..4)
            .map(|i| make_pest_finding(i + id as u32 * 4))
            .collect(),
        recommendations: vec![
            "Seal cracks around foundation".to_string(),
            "Remove standing water sources".to_string(),
            "Trim vegetation 18 inches from structure".to_string(),
        ],
        follow_up_days: 30,
    }
}

fn make_treatment_record(id: u64) -> TreatmentRecord {
    TreatmentRecord {
        record_id: id,
        report_id: id / 2 + 1,
        technician_id: 10000 + (id % 20) as u32,
        method: match id % 4 {
            0 => TreatmentMethod::ChemicalSpray,
            1 => TreatmentMethod::ChemicalBait,
            2 => TreatmentMethod::MechanicalTrap,
            _ => TreatmentMethod::BiologicalPredator,
        },
        product_name: format!("PestAway-{}", 100 + id),
        epa_registration: format!("EPA-{}-{}", 73049, 500 + id),
        active_ingredient: match id % 3 {
            0 => "Fipronil".to_string(),
            1 => "Imidacloprid".to_string(),
            _ => "Bifenthrin".to_string(),
        },
        concentration_ppm: 50 + id as u32 * 10,
        volume_applied_ml: 500 + id as u32 * 100,
        application_date_epoch_sec: 1_700_100_000 + id * 3600,
        target_pest: "German Cockroach".to_string(),
        wind_speed_kph_x10: 50 + (id % 30) as u16,
        temperature_c_x10: 220 + (id % 10) as i16,
        humidity_pct: 45 + (id % 40) as u8,
    }
}

fn make_bait_station(id: u32) -> BaitStationEntry {
    BaitStationEntry {
        station_id: id,
        latitude_microdeg: 33_950_000 + (id as i32 * 100),
        longitude_microdeg: -118_400_000 + (id as i32 * 150),
        status: match id % 5 {
            0 => BaitStationStatus::Active,
            1 => BaitStationStatus::NeedsRefill,
            2 => BaitStationStatus::Active,
            3 => BaitStationStatus::Damaged,
            _ => BaitStationStatus::Inactive,
        },
        bait_type: "First Strike Soft Bait".to_string(),
        bait_remaining_grams: 40 - (id % 35) as u16,
        consumption_grams: (id % 35) as u16,
        last_checked_epoch_sec: 1_700_200_000 + id as u64 * 86400,
        rodent_activity_score: (id % 10) as u8,
        notes: if id % 3 == 0 {
            "Heavy activity observed".to_string()
        } else {
            String::new()
        },
    }
}

fn make_heat_sensor_reading(sensor: u8, offset: u32) -> HeatSensorReading {
    HeatSensorReading {
        sensor_id: sensor,
        timestamp_offset_sec: offset,
        temperature_f_x10: 1300 + (offset % 200) as u16 + sensor as u16 * 5,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Round-trip for a single inspection report with multiple findings.
#[test]
fn test_zstd_inspection_report_roundtrip() {
    let report = make_inspection_report(1);
    let encoded = encode_to_vec(&report).expect("encode InspectionReport failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (InspectionReport, usize) =
        decode_from_slice(&decompressed).expect("decode InspectionReport failed");
    assert_eq!(report, decoded);
}

/// 2. Round-trip for a batch of treatment application records.
#[test]
fn test_zstd_treatment_records_batch_roundtrip() {
    let records: Vec<TreatmentRecord> = (1..=15).map(make_treatment_record).collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<TreatmentRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TreatmentRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<TreatmentRecord> failed");
    assert_eq!(records, decoded);
}

/// 3. Round-trip for bait station monitoring entries across a property.
#[test]
fn test_zstd_bait_station_monitoring_roundtrip() {
    let stations: Vec<BaitStationEntry> = (1..=24).map(make_bait_station).collect();
    let encoded = encode_to_vec(&stations).expect("encode Vec<BaitStationEntry> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BaitStationEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<BaitStationEntry> failed");
    assert_eq!(stations, decoded);
}

/// 4. Round-trip for a fumigation protocol with clearance readings.
#[test]
fn test_zstd_fumigation_protocol_roundtrip() {
    let protocol = FumigationProtocol {
        protocol_id: 7001,
        structure_volume_cuft: 45_000,
        fumigant: FumigantType::SulfurylFluoride,
        dosage_oz_per_1000cuft: 128,
        exposure_hours: 24,
        clearance_readings: vec![5, 3, 2, 1, 0, 0, 0, 0],
        tarp_seal_integrity_pct: 98,
        temperature_c_x10: 255,
        warning_placards_count: 12,
        aeration_start_epoch_sec: 1_700_300_000,
        aeration_end_epoch_sec: 1_700_343_600,
    };
    let encoded = encode_to_vec(&protocol).expect("encode FumigationProtocol failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (FumigationProtocol, usize) =
        decode_from_slice(&decompressed).expect("decode FumigationProtocol failed");
    assert_eq!(protocol, decoded);
}

/// 5. Round-trip for wildlife exclusion work with multiple entry points.
#[test]
fn test_zstd_wildlife_exclusion_roundtrip() {
    let work = WildlifeExclusionWork {
        work_order_id: 8001,
        target_species: "Eastern Gray Squirrel".to_string(),
        entry_points: vec![
            ExclusionPoint {
                location: "Soffit gap, north side".to_string(),
                opening_size_mm: 75,
                material_used: ExclusionMaterial::HardwareCloth,
                sealed: true,
            },
            ExclusionPoint {
                location: "Ridge vent gap".to_string(),
                opening_size_mm: 40,
                material_used: ExclusionMaterial::MetalFlashing,
                sealed: true,
            },
            ExclusionPoint {
                location: "Foundation crack, east wall".to_string(),
                opening_size_mm: 15,
                material_used: ExclusionMaterial::CopperMesh,
                sealed: true,
            },
            ExclusionPoint {
                location: "Garage door sweep".to_string(),
                opening_size_mm: 20,
                material_used: ExclusionMaterial::WeatherStripping,
                sealed: false,
            },
        ],
        total_materials_cost_cents: 34_500,
        labor_hours_x10: 65,
        warranty_months: 12,
    };
    let encoded = encode_to_vec(&work).expect("encode WildlifeExclusionWork failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (WildlifeExclusionWork, usize) =
        decode_from_slice(&decompressed).expect("decode WildlifeExclusionWork failed");
    assert_eq!(work, decoded);
}

/// 6. Round-trip for bed bug heat treatment with sensor time series.
#[test]
fn test_zstd_bed_bug_heat_treatment_roundtrip() {
    let mut readings = Vec::new();
    for sensor in 0..6u8 {
        for minute in (0..180).step_by(5) {
            readings.push(make_heat_sensor_reading(sensor, minute * 60));
        }
    }
    let treatment = BedBugHeatTreatment {
        treatment_id: 9001,
        room_sqft: 320,
        target_temp_f_x10: 1400,
        hold_duration_minutes: 120,
        sensor_readings: readings,
        prep_checklist_complete: true,
        heater_count: 3,
        fan_count: 4,
    };
    let encoded = encode_to_vec(&treatment).expect("encode BedBugHeatTreatment failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (BedBugHeatTreatment, usize) =
        decode_from_slice(&decompressed).expect("decode BedBugHeatTreatment failed");
    assert_eq!(treatment, decoded);
}

/// 7. Round-trip for termite colony tracking records.
#[test]
fn test_zstd_termite_colony_tracking_roundtrip() {
    let colony = TermiteColonyRecord {
        colony_id: 3001,
        species: "Reticulitermes flavipes".to_string(),
        estimated_population: 2_500_000,
        caste_counts: vec![
            (TermiteCasteType::Worker, 2_000_000),
            (TermiteCasteType::Soldier, 300_000),
            (TermiteCasteType::Reproductive, 50_000),
            (TermiteCasteType::Nymph, 149_998),
            (TermiteCasteType::Queen, 1),
            (TermiteCasteType::King, 1),
        ],
        monitoring_stations: vec![101, 102, 103, 104, 105, 106, 107, 108],
        bait_matrix_consumption_g: 450,
        first_detected_epoch_sec: 1_690_000_000,
        last_activity_epoch_sec: 1_700_400_000,
        colony_eliminated: false,
    };
    let encoded = encode_to_vec(&colony).expect("encode TermiteColonyRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TermiteColonyRecord, usize) =
        decode_from_slice(&decompressed).expect("decode TermiteColonyRecord failed");
    assert_eq!(colony, decoded);
}

/// 8. Round-trip for regulatory license records.
#[test]
fn test_zstd_regulatory_license_roundtrip() {
    let licenses = vec![
        RegulatoryLicense {
            license_number: "CA-SPC-2024-00451".to_string(),
            holder_name: "Jane Rodriguez".to_string(),
            license_type: LicenseType::StructuralPestControl,
            state_code: "CA".to_string(),
            issued_epoch_sec: 1_672_531_200,
            expires_epoch_sec: 1_704_067_200,
            continuing_education_hours: 36,
            violations_count: 0,
            active: true,
        },
        RegulatoryLicense {
            license_number: "FL-FUM-2023-01122".to_string(),
            holder_name: "Marcus Chen".to_string(),
            license_type: LicenseType::FumigationCertified,
            state_code: "FL".to_string(),
            issued_epoch_sec: 1_640_000_000,
            expires_epoch_sec: 1_703_000_000,
            continuing_education_hours: 48,
            violations_count: 1,
            active: true,
        },
        RegulatoryLicense {
            license_number: "TX-WM-2022-07890".to_string(),
            holder_name: "Aisha Patel".to_string(),
            license_type: LicenseType::WildlifeManagement,
            state_code: "TX".to_string(),
            issued_epoch_sec: 1_625_000_000,
            expires_epoch_sec: 1_688_000_000,
            continuing_education_hours: 24,
            violations_count: 0,
            active: false,
        },
    ];
    let encoded = encode_to_vec(&licenses).expect("encode Vec<RegulatoryLicense> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RegulatoryLicense>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<RegulatoryLicense> failed");
    assert_eq!(licenses, decoded);
}

/// 9. Round-trip for customer service agreements.
#[test]
fn test_zstd_service_agreement_roundtrip() {
    let agreement = ServiceAgreement {
        agreement_id: 50001,
        customer_id: 200_345,
        property_address_hash: 0x1234_5678_9ABC_DEF0,
        service_types: vec![
            "General Pest".to_string(),
            "Termite Monitoring".to_string(),
            "Rodent Exclusion".to_string(),
        ],
        monthly_fee_cents: 8_999,
        start_epoch_sec: 1_696_118_400,
        end_epoch_sec: 1_727_740_800,
        visit_frequency_days: 30,
        covered_pests: vec![
            "Ants".to_string(),
            "Cockroaches".to_string(),
            "Spiders".to_string(),
            "Silverfish".to_string(),
            "Mice".to_string(),
        ],
        exclusions: vec![
            "Bed bugs".to_string(),
            "Termite damage repair".to_string(),
            "Wildlife removal".to_string(),
        ],
        auto_renew: true,
    };
    let encoded = encode_to_vec(&agreement).expect("encode ServiceAgreement failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ServiceAgreement, usize) =
        decode_from_slice(&decompressed).expect("decode ServiceAgreement failed");
    assert_eq!(agreement, decoded);
}

/// 10. Round-trip for a rodent activity map with grid cells.
#[test]
fn test_zstd_rodent_activity_map_roundtrip() {
    let cells: Vec<RodentActivityCell> = (0..36)
        .map(|i| RodentActivityCell {
            grid_x: i % 6,
            grid_y: i / 6,
            species_detected: if i % 3 == 0 {
                vec![RodentSpecies::NorwayRat, RodentSpecies::HouseMouse]
            } else if i % 5 == 0 {
                vec![RodentSpecies::RoofRat]
            } else {
                vec![]
            },
            droppings_count: (i as u32 * 7) % 50,
            gnaw_marks: i % 4 == 0,
            burrow_entries: (i % 3) as u8,
            trail_camera_hits: (i * 2) % 15,
        })
        .collect();
    let map = RodentActivityMap {
        map_id: 6001,
        property_id: 200_345,
        grid_cols: 6,
        grid_rows: 6,
        cell_size_ft_x10: 100,
        survey_epoch_sec: 1_700_500_000,
        cells,
    };
    let encoded = encode_to_vec(&map).expect("encode RodentActivityMap failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (RodentActivityMap, usize) =
        decode_from_slice(&decompressed).expect("decode RodentActivityMap failed");
    assert_eq!(map, decoded);
}

/// 11. Round-trip for vegetation management schedule entries.
#[test]
fn test_zstd_vegetation_schedule_roundtrip() {
    let entries: Vec<VegetationScheduleEntry> = (1..=8)
        .map(|i| VegetationScheduleEntry {
            entry_id: 4000 + i,
            zone_name: format!("Perimeter Zone {}", (b'A' + (i as u8 - 1)) as char),
            area_sqft: 500 * i as u32,
            treatment: match i % 4 {
                0 => VegetationTreatmentType::PreEmergentHerbicide,
                1 => VegetationTreatmentType::Mowing,
                2 => VegetationTreatmentType::PostEmergentHerbicide,
                _ => VegetationTreatmentType::ManualRemoval,
            },
            product_name: if i % 2 == 0 {
                "ProScape Pre-M".to_string()
            } else {
                String::new()
            },
            application_rate_oz_per_acre_x10: 320 + i as u16 * 10,
            scheduled_epoch_sec: 1_700_600_000 + i * 604_800,
            completed: i <= 4,
            re_treatment_interval_days: 60 + (i % 3) as u16 * 30,
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode Vec<VegetationScheduleEntry> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<VegetationScheduleEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VegetationScheduleEntry> failed");
    assert_eq!(entries, decoded);
}

/// 12. Round-trip for an IPM strategy plan combining multiple control tactics.
#[test]
fn test_zstd_ipm_strategy_plan_roundtrip() {
    let plan = IpmStrategyPlan {
        plan_id: 11001,
        property_id: 200_345,
        target_pest: "Argentine Ant (Linepithema humile)".to_string(),
        category: PestCategory::Insect,
        action_thresholds: vec![
            ("Trailing ants per linear foot".to_string(), 5),
            ("Nesting sites within 10ft of structure".to_string(), 1),
            ("Indoor sightings per day".to_string(), 10),
        ],
        monitoring_interval_days: 14,
        cultural_controls: vec![
            "Remove mulch within 12 inches of foundation".to_string(),
            "Fix irrigation leaks".to_string(),
            "Store food in sealed containers".to_string(),
        ],
        biological_controls: vec!["Encourage native phorid fly populations".to_string()],
        chemical_controls: vec![
            "Gel bait stations along trails".to_string(),
            "Non-repellent perimeter treatment".to_string(),
        ],
        mechanical_controls: vec![
            "Seal cracks with caulk".to_string(),
            "Install door sweeps".to_string(),
        ],
        evaluation_date_epoch_sec: 1_701_000_000,
    };
    let encoded = encode_to_vec(&plan).expect("encode IpmStrategyPlan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (IpmStrategyPlan, usize) =
        decode_from_slice(&decompressed).expect("decode IpmStrategyPlan failed");
    assert_eq!(plan, decoded);
}

/// 13. Round-trip for equipment calibration records.
#[test]
fn test_zstd_equipment_calibration_roundtrip() {
    let calibrations: Vec<EquipmentCalibration> = (1..=6)
        .map(|i| EquipmentCalibration {
            equipment_id: 900 + i,
            equipment_type: match i % 3 {
                0 => "B&G Sprayer".to_string(),
                1 => "Power Duster".to_string(),
                _ => "Bait Gun".to_string(),
            },
            serial_number: format!("SN-2024-{:05}", i * 1111),
            calibration_date_epoch_sec: 1_700_000_000 + i as u64 * 86400,
            next_due_epoch_sec: 1_700_000_000 + i as u64 * 86400 + 7_776_000,
            flow_rate_ml_per_min_x10: 380 + i * 15,
            pressure_psi_x10: 400 + (i % 4) as u16 * 25,
            nozzle_pattern: match i % 2 {
                0 => "Fan 80-degree".to_string(),
                _ => "Pin stream".to_string(),
            },
            passed: i != 4,
        })
        .collect();
    let encoded = encode_to_vec(&calibrations).expect("encode Vec<EquipmentCalibration> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<EquipmentCalibration>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<EquipmentCalibration> failed");
    assert_eq!(calibrations, decoded);
}

/// 14. Compression reduces size for repetitive pest findings.
#[test]
fn test_zstd_compression_ratio_pest_findings() {
    let findings: Vec<PestFinding> = (0..100).map(make_pest_finding).collect();
    let encoded = encode_to_vec(&findings).expect("encode Vec<PestFinding> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "Compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<PestFinding>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PestFinding> failed");
    assert_eq!(findings, decoded);
}

/// 15. Round-trip for multiple inspection reports in sequence.
#[test]
fn test_zstd_multiple_inspection_reports_roundtrip() {
    let reports: Vec<InspectionReport> = (1..=10).map(make_inspection_report).collect();
    let encoded = encode_to_vec(&reports).expect("encode Vec<InspectionReport> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<InspectionReport>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<InspectionReport> failed");
    assert_eq!(reports, decoded);
}

/// 16. Round-trip for a fumigation protocol using phosphine gas.
#[test]
fn test_zstd_phosphine_fumigation_roundtrip() {
    let protocol = FumigationProtocol {
        protocol_id: 7050,
        structure_volume_cuft: 120_000,
        fumigant: FumigantType::PhosphineGas,
        dosage_oz_per_1000cuft: 200,
        exposure_hours: 72,
        clearance_readings: (0..16).map(|i| if i < 10 { 15 - i } else { 0 }).collect(),
        tarp_seal_integrity_pct: 95,
        temperature_c_x10: 280,
        warning_placards_count: 24,
        aeration_start_epoch_sec: 1_700_700_000,
        aeration_end_epoch_sec: 1_700_786_400,
    };
    let encoded = encode_to_vec(&protocol).expect("encode FumigationProtocol failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (FumigationProtocol, usize) =
        decode_from_slice(&decompressed).expect("decode FumigationProtocol failed");
    assert_eq!(protocol, decoded);
}

/// 17. Round-trip for a terminated termite colony.
#[test]
fn test_zstd_terminated_termite_colony_roundtrip() {
    let colony = TermiteColonyRecord {
        colony_id: 3099,
        species: "Coptotermes formosanus".to_string(),
        estimated_population: 0,
        caste_counts: vec![
            (TermiteCasteType::Worker, 0),
            (TermiteCasteType::Soldier, 0),
            (TermiteCasteType::Reproductive, 0),
            (TermiteCasteType::Nymph, 0),
            (TermiteCasteType::Queen, 0),
            (TermiteCasteType::King, 0),
        ],
        monitoring_stations: vec![201, 202, 203, 204, 205],
        bait_matrix_consumption_g: 1_200,
        first_detected_epoch_sec: 1_650_000_000,
        last_activity_epoch_sec: 1_695_000_000,
        colony_eliminated: true,
    };
    let encoded = encode_to_vec(&colony).expect("encode eliminated TermiteColonyRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TermiteColonyRecord, usize) =
        decode_from_slice(&decompressed).expect("decode eliminated TermiteColonyRecord failed");
    assert_eq!(colony, decoded);
}

/// 18. Round-trip for a service agreement with maximum pest coverage.
#[test]
fn test_zstd_comprehensive_service_agreement_roundtrip() {
    let agreement = ServiceAgreement {
        agreement_id: 50100,
        customer_id: 300_001,
        property_address_hash: 0xFEDC_BA98_7654_3210,
        service_types: vec![
            "General Pest".to_string(),
            "Termite Monitoring".to_string(),
            "Rodent Control".to_string(),
            "Mosquito Treatment".to_string(),
            "Fire Ant Control".to_string(),
            "Flea & Tick Treatment".to_string(),
        ],
        monthly_fee_cents: 24_999,
        start_epoch_sec: 1_700_000_000,
        end_epoch_sec: 1_731_600_000,
        visit_frequency_days: 14,
        covered_pests: (0..20)
            .map(|i| {
                [
                    "Fire Ants",
                    "Carpenter Ants",
                    "Pharaoh Ants",
                    "Odorous House Ants",
                    "German Cockroaches",
                    "American Cockroaches",
                    "Smoky Brown Cockroaches",
                    "Brown Recluse Spiders",
                    "Black Widow Spiders",
                    "Wolf Spiders",
                    "House Mice",
                    "Norway Rats",
                    "Roof Rats",
                    "Silverfish",
                    "Earwigs",
                    "Centipedes",
                    "Millipedes",
                    "Fleas",
                    "Ticks",
                    "Mosquitoes",
                ][i]
                    .to_string()
            })
            .collect(),
        exclusions: vec![
            "Bed bugs".to_string(),
            "Termite structural repair".to_string(),
        ],
        auto_renew: false,
    };
    let encoded = encode_to_vec(&agreement).expect("encode comprehensive ServiceAgreement failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ServiceAgreement, usize) =
        decode_from_slice(&decompressed).expect("decode comprehensive ServiceAgreement failed");
    assert_eq!(agreement, decoded);
}

/// 19. Round-trip for a large rodent activity map (12x12 grid).
#[test]
fn test_zstd_large_rodent_activity_map_roundtrip() {
    let cells: Vec<RodentActivityCell> = (0..144)
        .map(|i| {
            let x = i % 12;
            let y = i / 12;
            RodentActivityCell {
                grid_x: x,
                grid_y: y,
                species_detected: match (x + y) % 7 {
                    0 => vec![RodentSpecies::NorwayRat],
                    1 => vec![RodentSpecies::HouseMouse],
                    2 => vec![RodentSpecies::NorwayRat, RodentSpecies::HouseMouse],
                    3 => vec![RodentSpecies::RoofRat],
                    4 => vec![RodentSpecies::DeerMouse],
                    5 => vec![RodentSpecies::Vole, RodentSpecies::DeerMouse],
                    _ => vec![],
                },
                droppings_count: ((x as u32 * 13 + y as u32 * 7) % 120),
                gnaw_marks: (x + y) % 3 == 0,
                burrow_entries: ((x + y) % 5) as u8,
                trail_camera_hits: ((x * y) % 20),
            }
        })
        .collect();
    let map = RodentActivityMap {
        map_id: 6100,
        property_id: 400_001,
        grid_cols: 12,
        grid_rows: 12,
        cell_size_ft_x10: 50,
        survey_epoch_sec: 1_701_000_000,
        cells,
    };
    let encoded = encode_to_vec(&map).expect("encode large RodentActivityMap failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (RodentActivityMap, usize) =
        decode_from_slice(&decompressed).expect("decode large RodentActivityMap failed");
    assert_eq!(map, decoded);
}

/// 20. Round-trip for mixed exclusion materials in a raccoon exclusion job.
#[test]
fn test_zstd_raccoon_exclusion_multi_material_roundtrip() {
    let work = WildlifeExclusionWork {
        work_order_id: 8200,
        target_species: "Common Raccoon (Procyon lotor)".to_string(),
        entry_points: vec![
            ExclusionPoint {
                location: "Chimney cap missing".to_string(),
                opening_size_mm: 200,
                material_used: ExclusionMaterial::HardwareCloth,
                sealed: true,
            },
            ExclusionPoint {
                location: "Attic vent screen torn".to_string(),
                opening_size_mm: 150,
                material_used: ExclusionMaterial::HardwareCloth,
                sealed: true,
            },
            ExclusionPoint {
                location: "Crawlspace access unsecured".to_string(),
                opening_size_mm: 300,
                material_used: ExclusionMaterial::MetalFlashing,
                sealed: true,
            },
            ExclusionPoint {
                location: "Fascia board rotted section".to_string(),
                opening_size_mm: 100,
                material_used: ExclusionMaterial::MetalFlashing,
                sealed: true,
            },
            ExclusionPoint {
                location: "Plumbing penetration gap".to_string(),
                opening_size_mm: 50,
                material_used: ExclusionMaterial::ExpandingFoam,
                sealed: true,
            },
            ExclusionPoint {
                location: "Foundation vent screen missing".to_string(),
                opening_size_mm: 250,
                material_used: ExclusionMaterial::HardwareCloth,
                sealed: true,
            },
        ],
        total_materials_cost_cents: 89_500,
        labor_hours_x10: 120,
        warranty_months: 24,
    };
    let encoded = encode_to_vec(&work).expect("encode raccoon WildlifeExclusionWork failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (WildlifeExclusionWork, usize) =
        decode_from_slice(&decompressed).expect("decode raccoon WildlifeExclusionWork failed");
    assert_eq!(work, decoded);
}

/// 21. Round-trip for an IPM plan targeting fungal (mold) pest category.
#[test]
fn test_zstd_ipm_fungal_management_roundtrip() {
    let plan = IpmStrategyPlan {
        plan_id: 11050,
        property_id: 500_001,
        target_pest: "Wood Decay Fungi (Serpula lacrymans)".to_string(),
        category: PestCategory::Fungal,
        action_thresholds: vec![
            ("Moisture content percentage in framing".to_string(), 20),
            ("Visible fruiting body count".to_string(), 1),
            ("Affected area square feet".to_string(), 4),
        ],
        monitoring_interval_days: 30,
        cultural_controls: vec![
            "Improve crawlspace ventilation".to_string(),
            "Grade soil away from foundation".to_string(),
            "Repair roof leaks immediately".to_string(),
            "Install vapor barrier in crawlspace".to_string(),
        ],
        biological_controls: vec!["Trichoderma harzianum application to soil".to_string()],
        chemical_controls: vec![
            "Borate wood treatment on exposed framing".to_string(),
            "Copper naphthenate for cut ends".to_string(),
        ],
        mechanical_controls: vec![
            "Remove and replace damaged wood".to_string(),
            "Install dehumidifier in crawlspace".to_string(),
            "Add drainage to divert water".to_string(),
        ],
        evaluation_date_epoch_sec: 1_702_000_000,
    };
    let encoded = encode_to_vec(&plan).expect("encode fungal IpmStrategyPlan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (IpmStrategyPlan, usize) =
        decode_from_slice(&decompressed).expect("decode fungal IpmStrategyPlan failed");
    assert_eq!(plan, decoded);
}

/// 22. Round-trip for combined dataset: reports, treatments, and bait stations.
#[test]
fn test_zstd_combined_pest_control_dataset_roundtrip() {
    let reports: Vec<InspectionReport> = (1..=5).map(make_inspection_report).collect();
    let treatments: Vec<TreatmentRecord> = (1..=20).map(make_treatment_record).collect();
    let stations: Vec<BaitStationEntry> = (1..=30).map(make_bait_station).collect();

    let combined: (
        Vec<InspectionReport>,
        Vec<TreatmentRecord>,
        Vec<BaitStationEntry>,
    ) = (reports.clone(), treatments.clone(), stations.clone());

    let encoded = encode_to_vec(&combined).expect("encode combined dataset failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );

    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            Vec<InspectionReport>,
            Vec<TreatmentRecord>,
            Vec<BaitStationEntry>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode combined dataset failed");

    assert_eq!(combined.0, decoded.0, "InspectionReports mismatch");
    assert_eq!(combined.1, decoded.1, "TreatmentRecords mismatch");
    assert_eq!(combined.2, decoded.2, "BaitStationEntries mismatch");
}
