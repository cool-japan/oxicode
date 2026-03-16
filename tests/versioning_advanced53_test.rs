#![cfg(feature = "versioning")]

//! Versioning tests for OxiCode: beekeeping and apiary management systems.

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

// ── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QueenStatus {
    Laying,
    Virgin,
    SupersedureCells,
    Missing,
    Clipped,
    Marked,
    Drone,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BroodPattern {
    Solid,
    Spotty,
    Shotgun,
    Empty,
    DroneHeavy,
    Mixed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiseaseType {
    AmericanFoulbrood,
    EuropeanFoulbrood,
    Nosema,
    Chalkbrood,
    SacbroodVirus,
    Dwv,
    Varroosis,
    SmallHiveBeetle,
    WaxMoth,
    Healthy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColonyStrength {
    VeryStrong,
    Strong,
    Moderate,
    Weak,
    Critical,
    Dead,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeedType {
    SugarSyrup1To1,
    SugarSyrup2To1,
    FondantPatty,
    PollenSubstitute,
    ProteinPatty,
    HoneyFrame,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SwarmPreventionMethod {
    Split,
    Checkerboarding,
    Demaree,
    CutQueenCells,
    AddSuper,
    ReverseBroodBoxes,
    OpenMeshFloor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HarvestProduct {
    Honey,
    Wax,
    Propolis,
    RoyalJelly,
    Pollen,
    BeeVenom,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NectarFlowLevel {
    None,
    Light,
    Moderate,
    Heavy,
    Exceptional,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HiveInspectionRecord {
    hive_id: u32,
    apiary_name: String,
    inspector_name: String,
    day_of_year: u16,
    year: u16,
    temperature_celsius_x10: i16,
    queen_spotted: bool,
    queen_status: QueenStatus,
    brood_frames: u8,
    honey_frames: u8,
    pollen_frames: u8,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BroodAssessment {
    hive_id: u32,
    pattern: BroodPattern,
    capped_brood_percent: u8,
    larvae_visible: bool,
    eggs_visible: bool,
    drone_brood_percent: u8,
    queen_cells_count: u8,
    swarm_cells: bool,
    supersedure_cells: bool,
    estimated_bee_population: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HoneySuperManagement {
    hive_id: u32,
    supers_on: u8,
    supers_added: u8,
    supers_removed: u8,
    frames_per_super: u8,
    capped_percent: u8,
    estimated_weight_kg_x10: u32,
    queen_excluder_present: bool,
    super_type: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VarroaMiteCount {
    hive_id: u32,
    method: String,
    sample_size: u16,
    mite_count: u16,
    mites_per_hundred: u16,
    treatment_threshold_exceeded: bool,
    treatment_applied: Option<String>,
    treatment_date_day_of_year: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColonyStrengthRating {
    hive_id: u32,
    strength: ColonyStrength,
    frames_of_bees: u8,
    flight_activity_score: u8,
    temperament_score: u8,
    overwintering_potential: u8,
    combine_candidate: bool,
    requeen_recommended: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwarmPreventionRecord {
    hive_id: u32,
    method: SwarmPreventionMethod,
    day_of_year: u16,
    year: u16,
    queen_cells_destroyed: u8,
    frames_moved: u8,
    new_hive_created: bool,
    new_hive_id: Option<u32>,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NectarFlowPrediction {
    region: String,
    flora_sources: Vec<String>,
    expected_flow_level: NectarFlowLevel,
    bloom_start_day: u16,
    bloom_end_day: u16,
    historical_yield_kg_x10: u32,
    rainfall_mm_last30: u16,
    temperature_avg_x10: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestRecord {
    hive_id: u32,
    product: HarvestProduct,
    quantity_grams: u32,
    day_of_year: u16,
    year: u16,
    moisture_percent_x10: u16,
    color_grade: String,
    batch_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PollinationContract {
    contract_id: u64,
    grower_name: String,
    crop_type: String,
    field_hectares_x10: u32,
    hives_required: u16,
    hive_ids: Vec<u32>,
    start_day: u16,
    end_day: u16,
    year: u16,
    fee_per_hive_cents: u32,
    delivered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WinterPreparationChecklist {
    hive_id: u32,
    honey_stores_kg_x10: u32,
    mouse_guard_installed: bool,
    entrance_reducer_on: bool,
    ventilation_adequate: bool,
    varroa_treatment_completed: bool,
    queen_confirmed: bool,
    wrapping_applied: bool,
    candy_board_placed: bool,
    cluster_size_score: u8,
    wind_break_present: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiseaseDiagnosis {
    hive_id: u32,
    disease: DiseaseType,
    severity_score: u8,
    symptoms: Vec<String>,
    lab_confirmed: bool,
    sample_sent: bool,
    quarantine_required: bool,
    treatment_plan: String,
    affected_frames: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedingScheduleEntry {
    hive_id: u32,
    feed_type: FeedType,
    quantity_grams: u32,
    day_of_year: u16,
    year: u16,
    colony_accepted: bool,
    feeder_type: String,
    remaining_after_days: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ApiaryLayout {
    apiary_id: u32,
    name: String,
    latitude_x1000000: i64,
    longitude_x1000000: i64,
    elevation_meters: u16,
    hive_count: u16,
    hive_ids: Vec<u32>,
    sun_exposure_hours_x10: u16,
    water_source_distance_m: u32,
    bear_fence_installed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QueenRearingBatch {
    batch_id: u32,
    mother_queen_id: u32,
    grafting_day: u16,
    year: u16,
    cells_grafted: u16,
    cells_accepted: u16,
    queens_emerged: u16,
    queens_mated: u16,
    queens_laying: u16,
    lineage_notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HiveWeightLog {
    hive_id: u32,
    weight_grams: u32,
    day_of_year: u16,
    hour_of_day: u8,
    ambient_temp_x10: i16,
    humidity_percent: u8,
    rainfall_mm: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaxFoundationOrder {
    order_id: u64,
    supplier: String,
    frame_size: String,
    sheets_ordered: u16,
    wax_type: String,
    weight_per_sheet_grams: u16,
    total_cost_cents: u32,
    delivered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentInventory {
    item_name: String,
    category: String,
    quantity: u16,
    condition_score: u8,
    purchase_year: u16,
    replacement_needed: bool,
    cost_cents: u32,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_hive_inspection_record_versioned_roundtrip() {
    let record = HiveInspectionRecord {
        hive_id: 14,
        apiary_name: "Meadow Ridge Apiary".into(),
        inspector_name: "Elara Kowalski".into(),
        day_of_year: 142,
        year: 2026,
        temperature_celsius_x10: 225,
        queen_spotted: true,
        queen_status: QueenStatus::Laying,
        brood_frames: 7,
        honey_frames: 4,
        pollen_frames: 2,
        notes: "Strong colony, bees festooning between frames".into(),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&record, ver).expect("encode hive inspection");
    let (decoded, version, consumed): (HiveInspectionRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode hive inspection");
    assert_eq!(decoded, record);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_brood_assessment_with_queen_cells() {
    let assessment = BroodAssessment {
        hive_id: 7,
        pattern: BroodPattern::Solid,
        capped_brood_percent: 82,
        larvae_visible: true,
        eggs_visible: true,
        drone_brood_percent: 8,
        queen_cells_count: 3,
        swarm_cells: true,
        supersedure_cells: false,
        estimated_bee_population: 45000,
    };
    let ver = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&assessment, ver).expect("encode brood assessment");
    let (decoded, version, consumed): (BroodAssessment, Version, usize) =
        decode_versioned_value(&bytes).expect("decode brood assessment");
    assert_eq!(decoded, assessment);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_honey_super_management_multiple_supers() {
    let mgmt = HoneySuperManagement {
        hive_id: 22,
        supers_on: 3,
        supers_added: 1,
        supers_removed: 0,
        frames_per_super: 10,
        capped_percent: 75,
        estimated_weight_kg_x10: 280,
        queen_excluder_present: true,
        super_type: "Medium Langstroth".into(),
    };
    let ver = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&mgmt, ver).expect("encode super management");
    let (decoded, version, consumed): (HoneySuperManagement, Version, usize) =
        decode_versioned_value(&bytes).expect("decode super management");
    assert_eq!(decoded, mgmt);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_varroa_mite_count_threshold_exceeded() {
    let count = VarroaMiteCount {
        hive_id: 9,
        method: "alcohol wash".into(),
        sample_size: 300,
        mite_count: 12,
        mites_per_hundred: 4,
        treatment_threshold_exceeded: true,
        treatment_applied: Some("oxalic acid vaporization".into()),
        treatment_date_day_of_year: Some(245),
    };
    let ver = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&count, ver).expect("encode varroa count");
    let (decoded, version, consumed): (VarroaMiteCount, Version, usize) =
        decode_versioned_value(&bytes).expect("decode varroa count");
    assert_eq!(decoded, count);
    assert_eq!(version.major, 3);
    assert!(consumed > 0);
}

#[test]
fn test_varroa_mite_count_below_threshold() {
    let count = VarroaMiteCount {
        hive_id: 11,
        method: "sugar roll".into(),
        sample_size: 300,
        mite_count: 1,
        mites_per_hundred: 0,
        treatment_threshold_exceeded: false,
        treatment_applied: None,
        treatment_date_day_of_year: None,
    };
    let ver = Version::new(3, 0, 1);
    let bytes = encode_versioned_value(&count, ver).expect("encode varroa low count");
    let (decoded, version, consumed): (VarroaMiteCount, Version, usize) =
        decode_versioned_value(&bytes).expect("decode varroa low count");
    assert_eq!(decoded, count);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_colony_strength_rating_weak_colony() {
    let rating = ColonyStrengthRating {
        hive_id: 33,
        strength: ColonyStrength::Weak,
        frames_of_bees: 3,
        flight_activity_score: 2,
        temperament_score: 8,
        overwintering_potential: 3,
        combine_candidate: true,
        requeen_recommended: true,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&rating, ver).expect("encode colony strength");
    let (decoded, version, consumed): (ColonyStrengthRating, Version, usize) =
        decode_versioned_value(&bytes).expect("decode colony strength");
    assert_eq!(decoded, rating);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_swarm_prevention_with_split() {
    let prevention = SwarmPreventionRecord {
        hive_id: 5,
        method: SwarmPreventionMethod::Split,
        day_of_year: 128,
        year: 2026,
        queen_cells_destroyed: 4,
        frames_moved: 3,
        new_hive_created: true,
        new_hive_id: Some(51),
        success: true,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&prevention, ver).expect("encode swarm prevention");
    let (decoded, version, consumed): (SwarmPreventionRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode swarm prevention");
    assert_eq!(decoded, prevention);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

#[test]
fn test_swarm_prevention_checkerboarding_no_new_hive() {
    let prevention = SwarmPreventionRecord {
        hive_id: 18,
        method: SwarmPreventionMethod::Checkerboarding,
        day_of_year: 95,
        year: 2026,
        queen_cells_destroyed: 0,
        frames_moved: 5,
        new_hive_created: false,
        new_hive_id: None,
        success: true,
    };
    let ver = Version::new(2, 0, 1);
    let bytes = encode_versioned_value(&prevention, ver).expect("encode checkerboarding");
    let (decoded, version, consumed): (SwarmPreventionRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode checkerboarding");
    assert_eq!(decoded, prevention);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_nectar_flow_prediction_heavy_flow() {
    let prediction = NectarFlowPrediction {
        region: "Willamette Valley, OR".into(),
        flora_sources: vec![
            "blackberry".into(),
            "clover".into(),
            "fireweed".into(),
            "meadowfoam".into(),
        ],
        expected_flow_level: NectarFlowLevel::Heavy,
        bloom_start_day: 150,
        bloom_end_day: 220,
        historical_yield_kg_x10: 450,
        rainfall_mm_last30: 38,
        temperature_avg_x10: 218,
    };
    let ver = Version::new(1, 3, 0);
    let bytes = encode_versioned_value(&prediction, ver).expect("encode nectar flow");
    let (decoded, version, consumed): (NectarFlowPrediction, Version, usize) =
        decode_versioned_value(&bytes).expect("decode nectar flow");
    assert_eq!(decoded, prediction);
    assert_eq!(version.minor, 3);
    assert!(consumed > 0);
}

#[test]
fn test_harvest_record_honey_extraction() {
    let harvest = HarvestRecord {
        hive_id: 2,
        product: HarvestProduct::Honey,
        quantity_grams: 18500,
        day_of_year: 210,
        year: 2026,
        moisture_percent_x10: 172,
        color_grade: "Light Amber".into(),
        batch_id: "HA-2026-042".into(),
    };
    let ver = Version::new(4, 0, 0);
    let bytes = encode_versioned_value(&harvest, ver).expect("encode honey harvest");
    let (decoded, version, consumed): (HarvestRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode honey harvest");
    assert_eq!(decoded, harvest);
    assert_eq!(version.major, 4);
    assert!(consumed > 0);
}

#[test]
fn test_harvest_record_propolis_collection() {
    let harvest = HarvestRecord {
        hive_id: 8,
        product: HarvestProduct::Propolis,
        quantity_grams: 340,
        day_of_year: 270,
        year: 2026,
        moisture_percent_x10: 0,
        color_grade: "Dark Brown".into(),
        batch_id: "PR-2026-007".into(),
    };
    let ver = Version::new(4, 1, 0);
    let bytes = encode_versioned_value(&harvest, ver).expect("encode propolis harvest");
    let (decoded, version, consumed): (HarvestRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode propolis harvest");
    assert_eq!(decoded, harvest);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_pollination_contract_almond_groves() {
    let contract = PollinationContract {
        contract_id: 88001,
        grower_name: "Central Valley Almonds LLC".into(),
        crop_type: "almonds".into(),
        field_hectares_x10: 1200,
        hives_required: 48,
        hive_ids: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        start_day: 45,
        end_day: 85,
        year: 2026,
        fee_per_hive_cents: 22000,
        delivered: true,
    };
    let ver = Version::new(5, 0, 0);
    let bytes = encode_versioned_value(&contract, ver).expect("encode pollination contract");
    let (decoded, version, consumed): (PollinationContract, Version, usize) =
        decode_versioned_value(&bytes).expect("decode pollination contract");
    assert_eq!(decoded, contract);
    assert_eq!(version.major, 5);
    assert!(consumed > 0);
}

#[test]
fn test_winter_preparation_checklist_complete() {
    let checklist = WinterPreparationChecklist {
        hive_id: 6,
        honey_stores_kg_x10: 220,
        mouse_guard_installed: true,
        entrance_reducer_on: true,
        ventilation_adequate: true,
        varroa_treatment_completed: true,
        queen_confirmed: true,
        wrapping_applied: true,
        candy_board_placed: true,
        cluster_size_score: 8,
        wind_break_present: true,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&checklist, ver).expect("encode winter prep");
    let (decoded, version, consumed): (WinterPreparationChecklist, Version, usize) =
        decode_versioned_value(&bytes).expect("decode winter prep");
    assert_eq!(decoded, checklist);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_disease_diagnosis_afb_quarantine() {
    let diagnosis = DiseaseDiagnosis {
        hive_id: 19,
        disease: DiseaseType::AmericanFoulbrood,
        severity_score: 9,
        symptoms: vec![
            "ropy larval remains".into(),
            "foul smell".into(),
            "sunken cappings".into(),
            "perforated cappings".into(),
        ],
        lab_confirmed: true,
        sample_sent: true,
        quarantine_required: true,
        treatment_plan: "Burn hive and frames per state regulations; notify inspector".into(),
        affected_frames: 6,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&diagnosis, ver).expect("encode disease diagnosis");
    let (decoded, version, consumed): (DiseaseDiagnosis, Version, usize) =
        decode_versioned_value(&bytes).expect("decode disease diagnosis");
    assert_eq!(decoded, diagnosis);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

#[test]
fn test_disease_diagnosis_nosema_mild() {
    let diagnosis = DiseaseDiagnosis {
        hive_id: 27,
        disease: DiseaseType::Nosema,
        severity_score: 3,
        symptoms: vec![
            "dysentery spots on landing board".into(),
            "slow spring buildup".into(),
        ],
        lab_confirmed: false,
        sample_sent: false,
        quarantine_required: false,
        treatment_plan: "Monitor; ensure adequate ventilation; consider fumagillin if confirmed"
            .into(),
        affected_frames: 0,
    };
    let ver = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&diagnosis, ver).expect("encode nosema diagnosis");
    let (decoded, version, consumed): (DiseaseDiagnosis, Version, usize) =
        decode_versioned_value(&bytes).expect("decode nosema diagnosis");
    assert_eq!(decoded, diagnosis);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_feeding_schedule_fall_syrup() {
    let feeding = FeedingScheduleEntry {
        hive_id: 12,
        feed_type: FeedType::SugarSyrup2To1,
        quantity_grams: 5000,
        day_of_year: 260,
        year: 2026,
        colony_accepted: true,
        feeder_type: "top feeder".into(),
        remaining_after_days: 800,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&feeding, ver).expect("encode feeding schedule");
    let (decoded, version, consumed): (FeedingScheduleEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode feeding schedule");
    assert_eq!(decoded, feeding);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_apiary_layout_with_multiple_hives() {
    let layout = ApiaryLayout {
        apiary_id: 3,
        name: "Sunflower Hill Bee Yard".into(),
        latitude_x1000000: 44_943_200,
        longitude_x1000000: -123_035_100,
        elevation_meters: 85,
        hive_count: 6,
        hive_ids: vec![101, 102, 103, 104, 105, 106],
        sun_exposure_hours_x10: 72,
        water_source_distance_m: 150,
        bear_fence_installed: false,
    };
    let ver = Version::new(3, 2, 1);
    let bytes = encode_versioned_value(&layout, ver).expect("encode apiary layout");
    let (decoded, version, consumed): (ApiaryLayout, Version, usize) =
        decode_versioned_value(&bytes).expect("decode apiary layout");
    assert_eq!(decoded, layout);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_queen_rearing_batch_grafting_results() {
    let batch = QueenRearingBatch {
        batch_id: 17,
        mother_queen_id: 42,
        grafting_day: 135,
        year: 2026,
        cells_grafted: 30,
        cells_accepted: 22,
        queens_emerged: 19,
        queens_mated: 15,
        queens_laying: 13,
        lineage_notes: "VSH line from treatment-free survivor stock; gentle temperament".into(),
    };
    let ver = Version::new(1, 4, 0);
    let bytes = encode_versioned_value(&batch, ver).expect("encode queen rearing batch");
    let (decoded, version, consumed): (QueenRearingBatch, Version, usize) =
        decode_versioned_value(&bytes).expect("decode queen rearing batch");
    assert_eq!(decoded, batch);
    assert_eq!(version.minor, 4);
    assert!(consumed > 0);
}

#[test]
fn test_hive_weight_log_daily_reading() {
    let log_entry = HiveWeightLog {
        hive_id: 4,
        weight_grams: 42500,
        day_of_year: 175,
        hour_of_day: 14,
        ambient_temp_x10: 289,
        humidity_percent: 55,
        rainfall_mm: 0,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&log_entry, ver).expect("encode weight log");
    let (decoded, version, consumed): (HiveWeightLog, Version, usize) =
        decode_versioned_value(&bytes).expect("decode weight log");
    assert_eq!(decoded, log_entry);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_wax_foundation_order_roundtrip() {
    let order = WaxFoundationOrder {
        order_id: 550032,
        supplier: "Dadant & Sons".into(),
        frame_size: "deep Langstroth".into(),
        sheets_ordered: 200,
        wax_type: "pure beeswax".into(),
        weight_per_sheet_grams: 142,
        total_cost_cents: 48000,
        delivered: false,
    };
    let ver = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&order, ver).expect("encode wax foundation order");
    let (decoded, version, consumed): (WaxFoundationOrder, Version, usize) =
        decode_versioned_value(&bytes).expect("decode wax foundation order");
    assert_eq!(decoded, order);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_equipment_inventory_smoker_needs_replacement() {
    let item = EquipmentInventory {
        item_name: "Stainless Steel Smoker 4x7".into(),
        category: "protective equipment".into(),
        quantity: 2,
        condition_score: 3,
        purchase_year: 2019,
        replacement_needed: true,
        cost_cents: 4500,
    };
    let ver = Version::new(6, 0, 0);
    let bytes = encode_versioned_value(&item, ver).expect("encode equipment inventory");
    let (decoded, version, consumed): (EquipmentInventory, Version, usize) =
        decode_versioned_value(&bytes).expect("decode equipment inventory");
    assert_eq!(decoded, item);
    assert_eq!(version.major, 6);
    assert!(consumed > 0);
}

#[test]
fn test_brood_assessment_shotgun_pattern_critical_colony() {
    let assessment = BroodAssessment {
        hive_id: 41,
        pattern: BroodPattern::Shotgun,
        capped_brood_percent: 28,
        larvae_visible: true,
        eggs_visible: false,
        drone_brood_percent: 35,
        queen_cells_count: 0,
        swarm_cells: false,
        supersedure_cells: false,
        estimated_bee_population: 8000,
    };
    let rating = ColonyStrengthRating {
        hive_id: 41,
        strength: ColonyStrength::Critical,
        frames_of_bees: 2,
        flight_activity_score: 1,
        temperament_score: 5,
        overwintering_potential: 1,
        combine_candidate: true,
        requeen_recommended: true,
    };
    let combined: (BroodAssessment, ColonyStrengthRating) = (assessment.clone(), rating.clone());
    let ver = Version::new(2, 3, 0);
    let bytes = encode_versioned_value(&combined, ver).expect("encode combined assessment");
    let (decoded, version, consumed): ((BroodAssessment, ColonyStrengthRating), Version, usize) =
        decode_versioned_value(&bytes).expect("decode combined assessment");
    assert_eq!(decoded.0, assessment);
    assert_eq!(decoded.1, rating);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 3);
    assert!(consumed > 0);
}
