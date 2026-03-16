#![cfg(feature = "versioning")]

//! Versioning tests for OxiCode -- Theme: Aquaculture and fish farming operations.
//!
//! Covers 22 unique scenarios including fish pen configurations, water quality
//! monitoring, feeding schedules, growth sampling, disease tracking, harvest
//! planning, broodstock management, hatchery incubation, sea lice counts,
//! net pen cleaning, environmental impact, and stock density calculations.

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

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PenShape {
    Circular,
    Rectangular,
    Hexagonal,
    Octagonal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NetMaterial {
    Nylon,
    Polyethylene,
    CopperAlloy,
    Dyneema,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeedType {
    Pellet,
    Extruded,
    Moist,
    Live,
    Medicated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiseaseType {
    InfectiousSalmonAnemia,
    BacterialKidneyDisease,
    SeaLiceInfestation,
    AmoebicGillDisease,
    PancreaseDisease,
    ViralHemorrhagicSepticemia,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LifeStage {
    Egg,
    Alevin,
    Fry,
    Parr,
    Smolt,
    PostSmolt,
    Adult,
    Broodstock,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HarvestGrade {
    Superior,
    Ordinary,
    Production,
    Downgraded,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TreatmentMethod {
    BathTreatment,
    InFeedMedication,
    MechanicalDelousing,
    ThermalDelousing,
    LaserTreatment,
    CleanerFish,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FishPenConfig {
    pen_id: u32,
    site_name: String,
    shape: PenShape,
    circumference_m: u32,
    depth_m: u16,
    net_material: NetMaterial,
    mesh_size_mm: u16,
    max_biomass_kg: u64,
    gps_latitude_x1e6: i64,
    gps_longitude_x1e6: i64,
    installed_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterQualitySample {
    sample_id: u64,
    pen_id: u32,
    timestamp_epoch: u64,
    dissolved_oxygen_x100: u32,
    ph_x100: u16,
    salinity_ppt_x100: u32,
    temperature_c_x100: i32,
    turbidity_ntu_x100: u32,
    ammonia_mg_l_x1000: u32,
    nitrite_mg_l_x1000: u32,
    depth_of_measurement_m: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedingSchedule {
    schedule_id: u32,
    pen_id: u32,
    feed_type: FeedType,
    daily_ration_kg: u32,
    feeding_events_per_day: u8,
    pellet_size_mm_x10: u16,
    protein_pct_x10: u16,
    fat_pct_x10: u16,
    start_date_epoch: u64,
    end_date_epoch: u64,
    target_fcr_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GrowthSample {
    sample_id: u64,
    pen_id: u32,
    sample_date_epoch: u64,
    fish_count_sampled: u32,
    avg_weight_g: u32,
    std_dev_weight_g: u32,
    min_weight_g: u32,
    max_weight_g: u32,
    avg_length_mm: u32,
    condition_factor_x1000: u32,
    life_stage: LifeStage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiseaseOutbreakRecord {
    outbreak_id: u64,
    pen_id: u32,
    disease: DiseaseType,
    detection_date_epoch: u64,
    affected_fish_count: u32,
    mortality_count: u32,
    treatment: TreatmentMethod,
    treatment_start_epoch: u64,
    treatment_end_epoch: u64,
    quarantine_active: bool,
    veterinary_ref: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestPlan {
    plan_id: u32,
    pen_id: u32,
    planned_date_epoch: u64,
    estimated_fish_count: u64,
    estimated_biomass_kg: u64,
    target_weight_g: u32,
    grade: HarvestGrade,
    wellboat_name: String,
    processing_plant: String,
    transport_distance_km: u32,
    organic_certified: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BroodstockRecord {
    fish_tag_id: String,
    species: String,
    sex_male: bool,
    weight_g: u32,
    length_mm: u32,
    generation: u8,
    origin_hatchery: String,
    spawn_count: u16,
    last_spawn_epoch: u64,
    genetic_marker_hash: String,
    disease_free_certified: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HatcheryIncubationParams {
    batch_id: u64,
    species: String,
    egg_count: u64,
    water_temp_c_x100: i32,
    flow_rate_l_per_min_x10: u32,
    dissolved_oxygen_x100: u32,
    light_regime_hours_on: u8,
    light_regime_hours_off: u8,
    incubation_start_epoch: u64,
    expected_hatch_epoch: u64,
    eyed_egg_pct_x10: u16,
    survival_rate_pct_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaLiceCount {
    count_id: u64,
    pen_id: u32,
    sampling_date_epoch: u64,
    fish_sampled: u32,
    adult_female_avg_x100: u32,
    mobile_avg_x100: u32,
    chalimus_avg_x100: u32,
    total_lice_per_fish_x100: u32,
    threshold_exceeded: bool,
    treatment_required: bool,
    species_lepeophtheirus: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetPenCleaningSchedule {
    cleaning_id: u32,
    pen_id: u32,
    last_cleaned_epoch: u64,
    next_cleaning_epoch: u64,
    biofouling_level_pct: u8,
    method_in_situ: bool,
    diver_team_size: u8,
    estimated_duration_hours: u16,
    net_age_months: u16,
    replacement_needed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentalImpactAssessment {
    assessment_id: u64,
    site_name: String,
    date_epoch: u64,
    benthic_fauna_index_x100: u32,
    sediment_organic_c_pct_x100: u32,
    free_sulfide_um_x10: u32,
    current_speed_cm_s_x10: u32,
    current_direction_deg: u16,
    nitrogen_discharge_kg_per_ton: u32,
    phosphorus_discharge_kg_per_ton: u32,
    copper_concentration_ug_l_x10: u32,
    compliance_status: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StockDensityRecord {
    record_id: u64,
    pen_id: u32,
    date_epoch: u64,
    fish_count: u64,
    total_biomass_kg: u64,
    pen_volume_m3: u64,
    density_kg_per_m3_x100: u32,
    max_allowed_density_x100: u32,
    welfare_score: u8,
    smolt_input_date_epoch: u64,
    days_at_sea: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FcrCalculation {
    period_id: u32,
    pen_id: u32,
    period_start_epoch: u64,
    period_end_epoch: u64,
    total_feed_kg: u64,
    biomass_gain_kg: u64,
    fcr_x1000: u32,
    economic_fcr_x1000: u32,
    mortalities_kg: u64,
    feed_cost_per_kg_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OxygenMonitoringEvent {
    event_id: u64,
    pen_id: u32,
    timestamp_epoch: u64,
    surface_do_x100: u32,
    mid_depth_do_x100: u32,
    bottom_do_x100: u32,
    emergency_aeration_active: bool,
    supplemental_o2_kg: u32,
    alarm_triggered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortalityEvent {
    event_id: u64,
    pen_id: u32,
    date_epoch: u64,
    dead_fish_count: u32,
    estimated_weight_kg: u32,
    cause_description: String,
    disposal_method: String,
    cumulative_mortality_pct_x100: u32,
    reported_to_authority: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmoltTransferRecord {
    transfer_id: u64,
    source_hatchery: String,
    destination_pen_id: u32,
    transfer_date_epoch: u64,
    fish_count: u64,
    avg_weight_g: u32,
    vaccination_completed: bool,
    vaccine_batch_id: String,
    transport_duration_hours: u16,
    water_temp_during_transport_x100: i32,
    post_transfer_mortality_pct_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CleanerFishDeployment {
    deployment_id: u32,
    pen_id: u32,
    species: String,
    fish_count: u32,
    avg_weight_g: u32,
    deployment_date_epoch: u64,
    target_ratio_pct_x10: u16,
    hide_count: u16,
    supplemental_feed_g_per_day: u32,
    survival_rate_pct_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CurrentMeterReading {
    reading_id: u64,
    site_name: String,
    depth_m: u16,
    timestamp_epoch: u64,
    speed_cm_s_x100: u32,
    direction_deg_x10: u16,
    wave_height_cm: u32,
    wave_period_s_x10: u16,
    tidal_phase_deg: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FishWelfareScoringEvent {
    event_id: u64,
    pen_id: u32,
    date_epoch: u64,
    fish_inspected: u32,
    fin_damage_score_x10: u16,
    skin_lesion_score_x10: u16,
    eye_condition_score_x10: u16,
    gill_condition_score_x10: u16,
    operculum_score_x10: u16,
    overall_welfare_index_x100: u32,
    inspector_id: String,
}

// ── Test 1: Fish pen configuration roundtrip ─────────────────────────────────

#[test]
fn test_fish_pen_config_circular_v1() {
    let pen = FishPenConfig {
        pen_id: 101,
        site_name: "Nordfjorden Site A".to_string(),
        shape: PenShape::Circular,
        circumference_m: 160,
        depth_m: 35,
        net_material: NetMaterial::Nylon,
        mesh_size_mm: 25,
        max_biomass_kg: 780_000,
        gps_latitude_x1e6: 61_450_000,
        gps_longitude_x1e6: 5_320_000,
        installed_epoch: 1_672_531_200,
    };
    let version = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&pen, version).expect("encode FishPenConfig circular v1 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<FishPenConfig>(&bytes)
        .expect("decode FishPenConfig circular v1 failed");
    assert_eq!(pen, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

// ── Test 2: Water quality sample with low DO ─────────────────────────────────

#[test]
fn test_water_quality_low_dissolved_oxygen_v2() {
    let sample = WaterQualitySample {
        sample_id: 55_001,
        pen_id: 101,
        timestamp_epoch: 1_700_000_000,
        dissolved_oxygen_x100: 520,
        ph_x100: 780,
        salinity_ppt_x100: 3400,
        temperature_c_x100: 850,
        turbidity_ntu_x100: 120,
        ammonia_mg_l_x1000: 15,
        nitrite_mg_l_x1000: 8,
        depth_of_measurement_m: 10,
    };
    let version = Version::new(2, 1, 0);
    let bytes =
        encode_versioned_value(&sample, version).expect("encode WaterQualitySample low DO failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<WaterQualitySample>(&bytes)
        .expect("decode WaterQualitySample low DO failed");
    assert_eq!(sample, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
}

// ── Test 3: Feeding schedule with medicated feed ─────────────────────────────

#[test]
fn test_feeding_schedule_medicated_v1() {
    let schedule = FeedingSchedule {
        schedule_id: 3001,
        pen_id: 205,
        feed_type: FeedType::Medicated,
        daily_ration_kg: 4200,
        feeding_events_per_day: 6,
        pellet_size_mm_x10: 90,
        protein_pct_x10: 420,
        fat_pct_x10: 280,
        start_date_epoch: 1_704_067_200,
        end_date_epoch: 1_704_672_000,
        target_fcr_x100: 118,
    };
    let version = Version::new(1, 3, 0);
    let bytes = encode_versioned_value(&schedule, version)
        .expect("encode FeedingSchedule medicated failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<FeedingSchedule>(&bytes)
        .expect("decode FeedingSchedule medicated failed");
    assert_eq!(schedule, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
}

// ── Test 4: Growth sample at smolt stage ─────────────────────────────────────

#[test]
fn test_growth_sample_smolt_stage_v1() {
    let sample = GrowthSample {
        sample_id: 78_001,
        pen_id: 101,
        sample_date_epoch: 1_706_745_600,
        fish_count_sampled: 200,
        avg_weight_g: 4850,
        std_dev_weight_g: 620,
        min_weight_g: 3100,
        max_weight_g: 7200,
        avg_length_mm: 580,
        condition_factor_x1000: 1248,
        life_stage: LifeStage::Smolt,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sample, version).expect("encode GrowthSample smolt failed");
    let (decoded, ver, _consumed) =
        decode_versioned_value::<GrowthSample>(&bytes).expect("decode GrowthSample smolt failed");
    assert_eq!(sample, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 5: Disease outbreak ISA ─────────────────────────────────────────────

#[test]
fn test_disease_outbreak_isa_v3() {
    let outbreak = DiseaseOutbreakRecord {
        outbreak_id: 900_001,
        pen_id: 310,
        disease: DiseaseType::InfectiousSalmonAnemia,
        detection_date_epoch: 1_709_424_000,
        affected_fish_count: 1200,
        mortality_count: 340,
        treatment: TreatmentMethod::InFeedMedication,
        treatment_start_epoch: 1_709_510_400,
        treatment_end_epoch: 1_710_374_400,
        quarantine_active: true,
        veterinary_ref: "VET-NO-2024-0891".to_string(),
    };
    let version = Version::new(3, 0, 1);
    let bytes = encode_versioned_value(&outbreak, version)
        .expect("encode DiseaseOutbreakRecord ISA failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<DiseaseOutbreakRecord>(&bytes)
        .expect("decode DiseaseOutbreakRecord ISA failed");
    assert_eq!(outbreak, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.patch, 1);
}

// ── Test 6: Harvest plan with organic certification ──────────────────────────

#[test]
fn test_harvest_plan_organic_v2() {
    let plan = HarvestPlan {
        plan_id: 6001,
        pen_id: 101,
        planned_date_epoch: 1_711_929_600,
        estimated_fish_count: 185_000,
        estimated_biomass_kg: 925_000,
        target_weight_g: 5000,
        grade: HarvestGrade::Superior,
        wellboat_name: "MS Ronja Storm".to_string(),
        processing_plant: "Myre Processing AS".to_string(),
        transport_distance_km: 45,
        organic_certified: true,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&plan, version).expect("encode HarvestPlan organic failed");
    let (decoded, ver, _consumed) =
        decode_versioned_value::<HarvestPlan>(&bytes).expect("decode HarvestPlan organic failed");
    assert_eq!(plan, decoded);
    assert_eq!(ver.major, 2);
}

// ── Test 7: Broodstock record with genetic marker ────────────────────────────

#[test]
fn test_broodstock_record_male_v1() {
    let fish = BroodstockRecord {
        fish_tag_id: "PIT-NO-2023-004587".to_string(),
        species: "Salmo salar".to_string(),
        sex_male: true,
        weight_g: 12_400,
        length_mm: 980,
        generation: 5,
        origin_hatchery: "Bolaks Hatchery".to_string(),
        spawn_count: 3,
        last_spawn_epoch: 1_700_006_400,
        genetic_marker_hash: "a3f7c2d1e8b49056".to_string(),
        disease_free_certified: true,
    };
    let version = Version::new(1, 2, 0);
    let bytes =
        encode_versioned_value(&fish, version).expect("encode BroodstockRecord male failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<BroodstockRecord>(&bytes)
        .expect("decode BroodstockRecord male failed");
    assert_eq!(fish, decoded);
    assert_eq!(ver.minor, 2);
}

// ── Test 8: Hatchery incubation parameters ───────────────────────────────────

#[test]
fn test_hatchery_incubation_cold_water_v1() {
    let batch = HatcheryIncubationParams {
        batch_id: 44_001,
        species: "Oncorhynchus mykiss".to_string(),
        egg_count: 2_500_000,
        water_temp_c_x100: 680,
        flow_rate_l_per_min_x10: 1200,
        dissolved_oxygen_x100: 1050,
        light_regime_hours_on: 12,
        light_regime_hours_off: 12,
        incubation_start_epoch: 1_696_118_400,
        expected_hatch_epoch: 1_700_352_000,
        eyed_egg_pct_x10: 945,
        survival_rate_pct_x10: 872,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&batch, version)
        .expect("encode HatcheryIncubationParams cold water failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<HatcheryIncubationParams>(&bytes)
        .expect("decode HatcheryIncubationParams cold water failed");
    assert_eq!(batch, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 9: Sea lice count above threshold ───────────────────────────────────

#[test]
fn test_sea_lice_count_threshold_exceeded_v2() {
    let count = SeaLiceCount {
        count_id: 12_001,
        pen_id: 101,
        sampling_date_epoch: 1_710_288_000,
        fish_sampled: 20,
        adult_female_avg_x100: 85,
        mobile_avg_x100: 320,
        chalimus_avg_x100: 150,
        total_lice_per_fish_x100: 555,
        threshold_exceeded: true,
        treatment_required: true,
        species_lepeophtheirus: true,
    };
    let version = Version::new(2, 4, 0);
    let bytes = encode_versioned_value(&count, version)
        .expect("encode SeaLiceCount above threshold failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<SeaLiceCount>(&bytes)
        .expect("decode SeaLiceCount above threshold failed");
    assert_eq!(count, decoded);
    assert_eq!(ver.minor, 4);
}

// ── Test 10: Net pen cleaning schedule ───────────────────────────────────────

#[test]
fn test_net_pen_cleaning_high_biofouling_v1() {
    let schedule = NetPenCleaningSchedule {
        cleaning_id: 771,
        pen_id: 205,
        last_cleaned_epoch: 1_704_067_200,
        next_cleaning_epoch: 1_706_745_600,
        biofouling_level_pct: 72,
        method_in_situ: true,
        diver_team_size: 4,
        estimated_duration_hours: 8,
        net_age_months: 18,
        replacement_needed: false,
    };
    let version = Version::new(1, 1, 0);
    let bytes =
        encode_versioned_value(&schedule, version).expect("encode NetPenCleaningSchedule failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<NetPenCleaningSchedule>(&bytes)
        .expect("decode NetPenCleaningSchedule failed");
    assert_eq!(schedule, decoded);
    assert_eq!(ver.minor, 1);
}

// ── Test 11: Environmental impact assessment ─────────────────────────────────

#[test]
fn test_environmental_impact_compliant_site_v3() {
    let eia = EnvironmentalImpactAssessment {
        assessment_id: 2024_001,
        site_name: "Hardangerfjorden Site B".to_string(),
        date_epoch: 1_711_929_600,
        benthic_fauna_index_x100: 6800,
        sediment_organic_c_pct_x100: 320,
        free_sulfide_um_x10: 180,
        current_speed_cm_s_x10: 85,
        current_direction_deg: 225,
        nitrogen_discharge_kg_per_ton: 52,
        phosphorus_discharge_kg_per_ton: 9,
        copper_concentration_ug_l_x10: 34,
        compliance_status: true,
    };
    let version = Version::new(3, 1, 2);
    let bytes =
        encode_versioned_value(&eia, version).expect("encode EnvironmentalImpactAssessment failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<EnvironmentalImpactAssessment>(&bytes)
        .expect("decode EnvironmentalImpactAssessment failed");
    assert_eq!(eia, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 2);
}

// ── Test 12: Stock density record ────────────────────────────────────────────

#[test]
fn test_stock_density_within_limits_v1() {
    let record = StockDensityRecord {
        record_id: 33_001,
        pen_id: 101,
        date_epoch: 1_709_424_000,
        fish_count: 185_000,
        total_biomass_kg: 648_000,
        pen_volume_m3: 40_000,
        density_kg_per_m3_x100: 1620,
        max_allowed_density_x100: 2500,
        welfare_score: 4,
        smolt_input_date_epoch: 1_685_577_600,
        days_at_sea: 278,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&record, version).expect("encode StockDensityRecord failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<StockDensityRecord>(&bytes)
        .expect("decode StockDensityRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 13: FCR calculation ─────────────────────────────────────────────────

#[test]
fn test_fcr_calculation_good_conversion_v2() {
    let fcr = FcrCalculation {
        period_id: 401,
        pen_id: 101,
        period_start_epoch: 1_704_067_200,
        period_end_epoch: 1_706_745_600,
        total_feed_kg: 142_800,
        biomass_gain_kg: 119_000,
        fcr_x1000: 1200,
        economic_fcr_x1000: 1280,
        mortalities_kg: 9520,
        feed_cost_per_kg_x100: 1450,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&fcr, version).expect("encode FcrCalculation failed");
    let (decoded, ver, _consumed) =
        decode_versioned_value::<FcrCalculation>(&bytes).expect("decode FcrCalculation failed");
    assert_eq!(fcr, decoded);
    assert_eq!(ver.major, 2);
}

// ── Test 14: Oxygen monitoring emergency event ───────────────────────────────

#[test]
fn test_oxygen_monitoring_emergency_v1() {
    let event = OxygenMonitoringEvent {
        event_id: 88_001,
        pen_id: 310,
        timestamp_epoch: 1_710_547_200,
        surface_do_x100: 890,
        mid_depth_do_x100: 620,
        bottom_do_x100: 380,
        emergency_aeration_active: true,
        supplemental_o2_kg: 240,
        alarm_triggered: true,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&event, version)
        .expect("encode OxygenMonitoringEvent emergency failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<OxygenMonitoringEvent>(&bytes)
        .expect("decode OxygenMonitoringEvent emergency failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.minor, 5);
}

// ── Test 15: Mortality event record ──────────────────────────────────────────

#[test]
fn test_mortality_event_jellyfish_v2() {
    let event = MortalityEvent {
        event_id: 99_001,
        pen_id: 205,
        date_epoch: 1_711_152_000,
        dead_fish_count: 3400,
        estimated_weight_kg: 17_000,
        cause_description: "Pelagia noctiluca jellyfish bloom contact".to_string(),
        disposal_method: "Silage and rendering".to_string(),
        cumulative_mortality_pct_x100: 425,
        reported_to_authority: true,
    };
    let version = Version::new(2, 2, 0);
    let bytes =
        encode_versioned_value(&event, version).expect("encode MortalityEvent jellyfish failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<MortalityEvent>(&bytes)
        .expect("decode MortalityEvent jellyfish failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 2);
}

// ── Test 16: Smolt transfer record ───────────────────────────────────────────

#[test]
fn test_smolt_transfer_vaccinated_v1() {
    let transfer = SmoltTransferRecord {
        transfer_id: 7700_001,
        source_hatchery: "Laksevik Smolt AS".to_string(),
        destination_pen_id: 101,
        transfer_date_epoch: 1_685_577_600,
        fish_count: 200_000,
        avg_weight_g: 120,
        vaccination_completed: true,
        vaccine_batch_id: "VAX-ALPHA-2023-B12".to_string(),
        transport_duration_hours: 14,
        water_temp_during_transport_x100: 820,
        post_transfer_mortality_pct_x100: 35,
    };
    let version = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&transfer, version).expect("encode SmoltTransferRecord failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<SmoltTransferRecord>(&bytes)
        .expect("decode SmoltTransferRecord failed");
    assert_eq!(transfer, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 17: Cleaner fish deployment ─────────────────────────────────────────

#[test]
fn test_cleaner_fish_deployment_lumpfish_v2() {
    let deployment = CleanerFishDeployment {
        deployment_id: 550,
        pen_id: 101,
        species: "Cyclopterus lumpus".to_string(),
        fish_count: 9000,
        avg_weight_g: 45,
        deployment_date_epoch: 1_688_169_600,
        target_ratio_pct_x10: 50,
        hide_count: 180,
        supplemental_feed_g_per_day: 2700,
        survival_rate_pct_x10: 680,
    };
    let version = Version::new(2, 0, 1);
    let bytes = encode_versioned_value(&deployment, version)
        .expect("encode CleanerFishDeployment lumpfish failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<CleanerFishDeployment>(&bytes)
        .expect("decode CleanerFishDeployment lumpfish failed");
    assert_eq!(deployment, decoded);
    assert_eq!(ver.patch, 1);
}

// ── Test 18: Current meter reading ───────────────────────────────────────────

#[test]
fn test_current_meter_reading_strong_tidal_v1() {
    let reading = CurrentMeterReading {
        reading_id: 16_001,
        site_name: "Austevoll Outer".to_string(),
        depth_m: 15,
        timestamp_epoch: 1_710_633_600,
        speed_cm_s_x100: 4520,
        direction_deg_x10: 1850,
        wave_height_cm: 180,
        wave_period_s_x10: 72,
        tidal_phase_deg: 135,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&reading, version)
        .expect("encode CurrentMeterReading strong tidal failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<CurrentMeterReading>(&bytes)
        .expect("decode CurrentMeterReading strong tidal failed");
    assert_eq!(reading, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 19: Fish welfare scoring ────────────────────────────────────────────

#[test]
fn test_fish_welfare_scoring_good_condition_v2() {
    let event = FishWelfareScoringEvent {
        event_id: 22_001,
        pen_id: 101,
        date_epoch: 1_711_324_800,
        fish_inspected: 100,
        fin_damage_score_x10: 12,
        skin_lesion_score_x10: 8,
        eye_condition_score_x10: 5,
        gill_condition_score_x10: 10,
        operculum_score_x10: 3,
        overall_welfare_index_x100: 9240,
        inspector_id: "INS-NO-0042".to_string(),
    };
    let version = Version::new(2, 3, 0);
    let bytes =
        encode_versioned_value(&event, version).expect("encode FishWelfareScoringEvent failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<FishWelfareScoringEvent>(&bytes)
        .expect("decode FishWelfareScoringEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 3);
}

// ── Test 20: Vec of water quality samples ────────────────────────────────────

#[test]
fn test_vec_water_quality_samples_time_series_v1() {
    let samples: Vec<WaterQualitySample> = vec![
        WaterQualitySample {
            sample_id: 60_001,
            pen_id: 101,
            timestamp_epoch: 1_710_288_000,
            dissolved_oxygen_x100: 980,
            ph_x100: 810,
            salinity_ppt_x100: 3350,
            temperature_c_x100: 720,
            turbidity_ntu_x100: 80,
            ammonia_mg_l_x1000: 10,
            nitrite_mg_l_x1000: 5,
            depth_of_measurement_m: 5,
        },
        WaterQualitySample {
            sample_id: 60_002,
            pen_id: 101,
            timestamp_epoch: 1_710_291_600,
            dissolved_oxygen_x100: 940,
            ph_x100: 805,
            salinity_ppt_x100: 3340,
            temperature_c_x100: 730,
            turbidity_ntu_x100: 95,
            ammonia_mg_l_x1000: 12,
            nitrite_mg_l_x1000: 6,
            depth_of_measurement_m: 5,
        },
        WaterQualitySample {
            sample_id: 60_003,
            pen_id: 101,
            timestamp_epoch: 1_710_295_200,
            dissolved_oxygen_x100: 870,
            ph_x100: 798,
            salinity_ppt_x100: 3320,
            temperature_c_x100: 745,
            turbidity_ntu_x100: 110,
            ammonia_mg_l_x1000: 18,
            nitrite_mg_l_x1000: 9,
            depth_of_measurement_m: 5,
        },
    ];
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&samples, version)
        .expect("encode Vec<WaterQualitySample> time series failed");
    let (decoded, ver, _consumed): (Vec<WaterQualitySample>, _, _) =
        decode_versioned_value(&bytes).expect("decode Vec<WaterQualitySample> time series failed");
    assert_eq!(samples, decoded);
    assert_eq!(ver.major, 1);
}

// ── Test 21: Hexagonal pen with copper alloy net ─────────────────────────────

#[test]
fn test_fish_pen_hexagonal_copper_alloy_v4() {
    let pen = FishPenConfig {
        pen_id: 602,
        site_name: "Troms Offshore Farm".to_string(),
        shape: PenShape::Hexagonal,
        circumference_m: 200,
        depth_m: 50,
        net_material: NetMaterial::CopperAlloy,
        mesh_size_mm: 30,
        max_biomass_kg: 1_200_000,
        gps_latitude_x1e6: 69_650_000,
        gps_longitude_x1e6: 18_950_000,
        installed_epoch: 1_688_169_600,
    };
    let version = Version::new(4, 0, 0);
    let bytes = encode_versioned_value(&pen, version)
        .expect("encode FishPenConfig hexagonal copper alloy failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<FishPenConfig>(&bytes)
        .expect("decode FishPenConfig hexagonal copper alloy failed");
    assert_eq!(pen, decoded);
    assert_eq!(ver.major, 4);
}

// ── Test 22: Vec of disease outbreaks across pens ────────────────────────────

#[test]
fn test_vec_disease_outbreaks_multi_pen_v2() {
    let outbreaks: Vec<DiseaseOutbreakRecord> = vec![
        DiseaseOutbreakRecord {
            outbreak_id: 900_010,
            pen_id: 101,
            disease: DiseaseType::SeaLiceInfestation,
            detection_date_epoch: 1_711_065_600,
            affected_fish_count: 185_000,
            mortality_count: 0,
            treatment: TreatmentMethod::MechanicalDelousing,
            treatment_start_epoch: 1_711_152_000,
            treatment_end_epoch: 1_711_238_400,
            quarantine_active: false,
            veterinary_ref: "VET-NO-2024-1102".to_string(),
        },
        DiseaseOutbreakRecord {
            outbreak_id: 900_011,
            pen_id: 205,
            disease: DiseaseType::AmoebicGillDisease,
            detection_date_epoch: 1_711_152_000,
            affected_fish_count: 42_000,
            mortality_count: 850,
            treatment: TreatmentMethod::BathTreatment,
            treatment_start_epoch: 1_711_238_400,
            treatment_end_epoch: 1_711_843_200,
            quarantine_active: true,
            veterinary_ref: "VET-NO-2024-1105".to_string(),
        },
    ];
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&outbreaks, version)
        .expect("encode Vec<DiseaseOutbreakRecord> multi pen failed");
    let (decoded, ver, _consumed): (Vec<DiseaseOutbreakRecord>, _, _) =
        decode_versioned_value(&bytes).expect("decode Vec<DiseaseOutbreakRecord> multi pen failed");
    assert_eq!(outbreaks, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
}
