#![cfg(feature = "std")]
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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ── Domain Types: Forestry Management & Wildfire Prevention ─────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreeSpecies {
    common_name: String,
    scientific_name: String,
    fire_resistance_rating: u8,
    is_conifer: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimberInventoryRecord {
    plot_id: u32,
    species: TreeSpecies,
    dbh_cm: f64,
    height_m: f64,
    volume_m3: f64,
    age_years: u16,
    merchantable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestType {
    BarkBeetle,
    Defoliator,
    RootDisease,
    DwarfMistletoe,
    WhitePineBlisterRust,
    SuddenOakDeath,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ForestHealthSurvey {
    survey_id: u64,
    plot_id: u32,
    pest_detected: PestType,
    severity_percent: f64,
    affected_tree_count: u32,
    treatment_recommended: String,
    surveyor_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelMoistureReading {
    station_id: u32,
    one_hour_percent: f64,
    ten_hour_percent: f64,
    hundred_hour_percent: f64,
    thousand_hour_percent: f64,
    live_herbaceous_percent: f64,
    live_woody_percent: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TopographyClass {
    Flat,
    GentleSlope,
    ModerateSlope,
    SteepSlope,
    CliffFace,
    Canyon,
    Ridgeline,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WildfireRiskIndex {
    zone_id: u32,
    fuel_moisture: FuelMoistureReading,
    wind_speed_kmh: f64,
    wind_direction_deg: u16,
    topography: TopographyClass,
    drought_index: f64,
    overall_risk_score: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FireBehaviorModel {
    model_id: u64,
    fuel_model_code: String,
    rate_of_spread_m_per_min: f64,
    flame_length_m: f64,
    fireline_intensity_kw_per_m: f64,
    spotting_distance_km: f64,
    crown_fire_potential: bool,
    predicted_area_ha: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BurnObjective {
    FuelReduction,
    HabitatRestoration,
    SilviculturalTreatment,
    InvasiveSpeciesControl,
    RangeImprovement,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrescribedBurnPlan {
    plan_id: u64,
    unit_name: String,
    area_ha: f64,
    objective: BurnObjective,
    min_wind_speed_kmh: f64,
    max_wind_speed_kmh: f64,
    min_relative_humidity: f64,
    max_temperature_c: f64,
    ignition_pattern: String,
    contingency_lines_km: f64,
    personnel_required: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ResourceType {
    Engine(String),
    Hotshot,
    Helicopter(u32),
    AirTanker(u32),
    Dozer,
    WaterTender,
    HandCrew(u16),
    CommandPost,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FirefightingResource {
    resource_id: u64,
    resource_type: ResourceType,
    assigned_division: String,
    status_available: bool,
    lat: f64,
    lon: f64,
    personnel_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmokeDispersionForecast {
    forecast_id: u64,
    source_fire_name: String,
    pm25_ug_per_m3: f64,
    pm10_ug_per_m3: f64,
    visibility_km: f64,
    plume_height_m: f64,
    wind_transport_direction_deg: u16,
    affected_communities: Vec<String>,
    aqi_category: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RehabilitationMethod {
    ContourFelling,
    MulchApplication,
    SeedMix(String),
    ErosionBarrier,
    StreamCrossing,
    HydroSeeding,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PostFireRehabilitation {
    project_id: u64,
    fire_name: String,
    area_ha: f64,
    burn_severity_class: u8,
    methods: Vec<RehabilitationMethod>,
    estimated_cost_usd: f64,
    erosion_risk_high: bool,
    watershed_priority: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarbonSequestrationEstimate {
    stand_id: u32,
    above_ground_biomass_tonnes_per_ha: f64,
    below_ground_biomass_tonnes_per_ha: f64,
    dead_wood_tonnes_per_ha: f64,
    litter_tonnes_per_ha: f64,
    soil_organic_carbon_tonnes_per_ha: f64,
    total_carbon_tonnes_per_ha: f64,
    annual_sequestration_rate: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HabitatQuality {
    Pristine,
    Good,
    Moderate,
    Degraded,
    Destroyed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WildlifeHabitatAssessment {
    assessment_id: u64,
    species_of_concern: String,
    habitat_type: String,
    quality: HabitatQuality,
    patch_size_ha: f64,
    connectivity_index: f64,
    snag_density_per_ha: f64,
    canopy_closure_percent: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WatershedProtectionZone {
    zone_id: u32,
    watershed_name: String,
    area_ha: f64,
    buffer_width_m: f64,
    stream_order: u8,
    is_municipal_supply: bool,
    sediment_yield_tonnes_per_yr: f64,
    logging_restricted: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoadCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Impassable,
    Decommissioned,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoggingRoad {
    road_id: u32,
    road_name: String,
    length_km: f64,
    surface_type: String,
    condition: RoadCondition,
    max_gross_vehicle_weight_kg: u32,
    drainage_structures_count: u16,
    seasonal_closure: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CanopyAnalysis {
    tile_id: u64,
    canopy_cover_percent: f64,
    canopy_height_mean_m: f64,
    canopy_height_max_m: f64,
    leaf_area_index: f64,
    ndvi_mean: f64,
    gap_fraction_percent: f64,
    sensor_platform: String,
    resolution_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReforestationPlan {
    plan_id: u64,
    site_name: String,
    area_ha: f64,
    target_species: Vec<TreeSpecies>,
    seedlings_per_ha: u32,
    planting_year: u16,
    site_prep_method: String,
    expected_survival_percent: f64,
    monitoring_years: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentStatus {
    Operational,
    MaintenanceDue,
    InRepair,
    Retired,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestEquipmentLog {
    equipment_id: u64,
    equipment_type: String,
    serial_number: String,
    hours_operated: f64,
    fuel_consumed_liters: f64,
    volume_harvested_m3: f64,
    status: EquipmentStatus,
    operator_name: String,
    last_service_date: String,
}

// ── Helper ──────────────────────────────────────────────────────────────────

fn test_path(suffix: &str) -> std::path::PathBuf {
    temp_dir().join(format!(
        "oxicode_fio41_{}_{}.bin",
        suffix,
        std::process::id()
    ))
}

// ── Test 1: Timber inventory record roundtrip via file ──────────────────────

#[test]
fn test_timber_inventory_file_roundtrip() {
    let path = test_path("t01");
    let record = TimberInventoryRecord {
        plot_id: 4201,
        species: TreeSpecies {
            common_name: "Douglas Fir".into(),
            scientific_name: "Pseudotsuga menziesii".into(),
            fire_resistance_rating: 7,
            is_conifer: true,
        },
        dbh_cm: 48.3,
        height_m: 32.7,
        volume_m3: 3.14,
        age_years: 85,
        merchantable: true,
    };
    encode_to_file(&record, &path).expect("encode timber inventory to file");
    let decoded: TimberInventoryRecord =
        decode_from_file(&path).expect("decode timber inventory from file");
    assert_eq!(record, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: Multiple timber records via vec slice ───────────────────────────

#[test]
fn test_timber_inventory_batch_vec_slice() {
    let records: Vec<TimberInventoryRecord> = vec![
        TimberInventoryRecord {
            plot_id: 1001,
            species: TreeSpecies {
                common_name: "Ponderosa Pine".into(),
                scientific_name: "Pinus ponderosa".into(),
                fire_resistance_rating: 8,
                is_conifer: true,
            },
            dbh_cm: 55.0,
            height_m: 28.0,
            volume_m3: 4.2,
            age_years: 120,
            merchantable: true,
        },
        TimberInventoryRecord {
            plot_id: 1001,
            species: TreeSpecies {
                common_name: "White Fir".into(),
                scientific_name: "Abies concolor".into(),
                fire_resistance_rating: 3,
                is_conifer: true,
            },
            dbh_cm: 22.0,
            height_m: 15.5,
            volume_m3: 0.6,
            age_years: 40,
            merchantable: false,
        },
    ];
    let bytes = encode_to_vec(&records).expect("encode timber batch");
    let (decoded, _): (Vec<TimberInventoryRecord>, usize) =
        decode_from_slice(&bytes).expect("decode timber batch");
    assert_eq!(records, decoded);
}

// ── Test 3: Forest health survey with pest detection ────────────────────────

#[test]
fn test_forest_health_survey_file() {
    let path = test_path("t03");
    let survey = ForestHealthSurvey {
        survey_id: 88001,
        plot_id: 3300,
        pest_detected: PestType::BarkBeetle,
        severity_percent: 34.5,
        affected_tree_count: 127,
        treatment_recommended: "Salvage harvest and pheromone trapping".into(),
        surveyor_name: "Maria Gonzalez".into(),
    };
    encode_to_file(&survey, &path).expect("encode health survey");
    let decoded: ForestHealthSurvey = decode_from_file(&path).expect("decode health survey");
    assert_eq!(survey, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: Multiple pest types via vec slice ───────────────────────────────

#[test]
fn test_all_pest_types_roundtrip() {
    let pests = vec![
        PestType::BarkBeetle,
        PestType::Defoliator,
        PestType::RootDisease,
        PestType::DwarfMistletoe,
        PestType::WhitePineBlisterRust,
        PestType::SuddenOakDeath,
    ];
    let bytes = encode_to_vec(&pests).expect("encode pest types");
    let (decoded, _): (Vec<PestType>, usize) =
        decode_from_slice(&bytes).expect("decode pest types");
    assert_eq!(pests, decoded);
}

// ── Test 5: Wildfire risk index with fuel moisture ──────────────────────────

#[test]
fn test_wildfire_risk_index_file() {
    let path = test_path("t05");
    let risk = WildfireRiskIndex {
        zone_id: 7700,
        fuel_moisture: FuelMoistureReading {
            station_id: 501,
            one_hour_percent: 3.2,
            ten_hour_percent: 5.8,
            hundred_hour_percent: 9.1,
            thousand_hour_percent: 14.7,
            live_herbaceous_percent: 45.0,
            live_woody_percent: 78.0,
        },
        wind_speed_kmh: 42.0,
        wind_direction_deg: 225,
        topography: TopographyClass::SteepSlope,
        drought_index: 650.0,
        overall_risk_score: 92.3,
    };
    encode_to_file(&risk, &path).expect("encode wildfire risk");
    let decoded: WildfireRiskIndex = decode_from_file(&path).expect("decode wildfire risk");
    assert_eq!(risk, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: Topography class variants via vec slice ─────────────────────────

#[test]
fn test_topography_classes_roundtrip() {
    let classes = vec![
        TopographyClass::Flat,
        TopographyClass::GentleSlope,
        TopographyClass::ModerateSlope,
        TopographyClass::SteepSlope,
        TopographyClass::CliffFace,
        TopographyClass::Canyon,
        TopographyClass::Ridgeline,
    ];
    let bytes = encode_to_vec(&classes).expect("encode topography");
    let (decoded, _): (Vec<TopographyClass>, usize) =
        decode_from_slice(&bytes).expect("decode topography");
    assert_eq!(classes, decoded);
}

// ── Test 7: Fire behavior model via file ────────────────────────────────────

#[test]
fn test_fire_behavior_model_file() {
    let path = test_path("t07");
    let model = FireBehaviorModel {
        model_id: 20260315001,
        fuel_model_code: "TL8".into(),
        rate_of_spread_m_per_min: 12.5,
        flame_length_m: 3.8,
        fireline_intensity_kw_per_m: 4200.0,
        spotting_distance_km: 1.2,
        crown_fire_potential: true,
        predicted_area_ha: 340.0,
    };
    encode_to_file(&model, &path).expect("encode fire behavior model");
    let decoded: FireBehaviorModel = decode_from_file(&path).expect("decode fire behavior model");
    assert_eq!(model, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: Prescribed burn plan via vec slice ──────────────────────────────

#[test]
fn test_prescribed_burn_plan_vec_slice() {
    let plan = PrescribedBurnPlan {
        plan_id: 550001,
        unit_name: "North Ridge Unit 7B".into(),
        area_ha: 120.0,
        objective: BurnObjective::FuelReduction,
        min_wind_speed_kmh: 8.0,
        max_wind_speed_kmh: 24.0,
        min_relative_humidity: 25.0,
        max_temperature_c: 30.0,
        ignition_pattern: "Strip head fire with backing ignition".into(),
        contingency_lines_km: 4.5,
        personnel_required: 28,
    };
    let bytes = encode_to_vec(&plan).expect("encode burn plan");
    let (decoded, _): (PrescribedBurnPlan, usize) =
        decode_from_slice(&bytes).expect("decode burn plan");
    assert_eq!(plan, decoded);
}

// ── Test 9: Burn objectives enum coverage ───────────────────────────────────

#[test]
fn test_burn_objectives_all_variants_file() {
    let path = test_path("t09");
    let objectives = vec![
        BurnObjective::FuelReduction,
        BurnObjective::HabitatRestoration,
        BurnObjective::SilviculturalTreatment,
        BurnObjective::InvasiveSpeciesControl,
        BurnObjective::RangeImprovement,
    ];
    encode_to_file(&objectives, &path).expect("encode objectives");
    let decoded: Vec<BurnObjective> = decode_from_file(&path).expect("decode objectives");
    assert_eq!(objectives, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Firefighting resource deployment via file ──────────────────────

#[test]
fn test_firefighting_resource_deployment_file() {
    let path = test_path("t10");
    let resources = vec![
        FirefightingResource {
            resource_id: 10001,
            resource_type: ResourceType::Engine("Type 3".into()),
            assigned_division: "Division Alpha".into(),
            status_available: true,
            lat: 39.7392,
            lon: -104.9903,
            personnel_count: 4,
        },
        FirefightingResource {
            resource_id: 10002,
            resource_type: ResourceType::Hotshot,
            assigned_division: "Division Bravo".into(),
            status_available: false,
            lat: 39.7510,
            lon: -105.0020,
            personnel_count: 20,
        },
        FirefightingResource {
            resource_id: 10003,
            resource_type: ResourceType::Helicopter(205),
            assigned_division: "Air Operations".into(),
            status_available: true,
            lat: 39.7800,
            lon: -104.9500,
            personnel_count: 3,
        },
        FirefightingResource {
            resource_id: 10004,
            resource_type: ResourceType::AirTanker(900),
            assigned_division: "Air Operations".into(),
            status_available: true,
            lat: 39.8100,
            lon: -104.8800,
            personnel_count: 2,
        },
        FirefightingResource {
            resource_id: 10005,
            resource_type: ResourceType::HandCrew(20),
            assigned_division: "Division Charlie".into(),
            status_available: true,
            lat: 39.7650,
            lon: -105.0100,
            personnel_count: 20,
        },
    ];
    encode_to_file(&resources, &path).expect("encode resources");
    let decoded: Vec<FirefightingResource> = decode_from_file(&path).expect("decode resources");
    assert_eq!(resources, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 11: Resource type enum variant coverage ────────────────────────────

#[test]
fn test_resource_type_variants_vec_slice() {
    let types = vec![
        ResourceType::Engine("Type 1".into()),
        ResourceType::Hotshot,
        ResourceType::Helicopter(412),
        ResourceType::AirTanker(130),
        ResourceType::Dozer,
        ResourceType::WaterTender,
        ResourceType::HandCrew(10),
        ResourceType::CommandPost,
    ];
    let bytes = encode_to_vec(&types).expect("encode resource types");
    let (decoded, _): (Vec<ResourceType>, usize) =
        decode_from_slice(&bytes).expect("decode resource types");
    assert_eq!(types, decoded);
}

// ── Test 12: Smoke dispersion forecast via file ─────────────────────────────

#[test]
fn test_smoke_dispersion_forecast_file() {
    let path = test_path("t12");
    let forecast = SmokeDispersionForecast {
        forecast_id: 660042,
        source_fire_name: "Pine Creek Complex".into(),
        pm25_ug_per_m3: 185.3,
        pm10_ug_per_m3: 240.0,
        visibility_km: 1.5,
        plume_height_m: 3200.0,
        wind_transport_direction_deg: 270,
        affected_communities: vec![
            "Cedarville".into(),
            "Eagle Point".into(),
            "Timber Ridge".into(),
            "Lakeview Estates".into(),
        ],
        aqi_category: "Very Unhealthy".into(),
    };
    encode_to_file(&forecast, &path).expect("encode smoke forecast");
    let decoded: SmokeDispersionForecast = decode_from_file(&path).expect("decode smoke forecast");
    assert_eq!(forecast, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 13: Post-fire rehabilitation via vec slice ─────────────────────────

#[test]
fn test_post_fire_rehabilitation_vec_slice() {
    let project = PostFireRehabilitation {
        project_id: 990001,
        fire_name: "Thunderbolt Fire".into(),
        area_ha: 2800.0,
        burn_severity_class: 3,
        methods: vec![
            RehabilitationMethod::ContourFelling,
            RehabilitationMethod::MulchApplication,
            RehabilitationMethod::SeedMix("Native grass/forb mix #12".into()),
            RehabilitationMethod::ErosionBarrier,
            RehabilitationMethod::StreamCrossing,
            RehabilitationMethod::HydroSeeding,
        ],
        estimated_cost_usd: 4_250_000.0,
        erosion_risk_high: true,
        watershed_priority: true,
    };
    let bytes = encode_to_vec(&project).expect("encode rehabilitation");
    let (decoded, _): (PostFireRehabilitation, usize) =
        decode_from_slice(&bytes).expect("decode rehabilitation");
    assert_eq!(project, decoded);
}

// ── Test 14: Carbon sequestration estimates via file ────────────────────────

#[test]
fn test_carbon_sequestration_file() {
    let path = test_path("t14");
    let estimate = CarbonSequestrationEstimate {
        stand_id: 8800,
        above_ground_biomass_tonnes_per_ha: 185.4,
        below_ground_biomass_tonnes_per_ha: 46.3,
        dead_wood_tonnes_per_ha: 22.1,
        litter_tonnes_per_ha: 11.5,
        soil_organic_carbon_tonnes_per_ha: 98.7,
        total_carbon_tonnes_per_ha: 364.0,
        annual_sequestration_rate: 4.2,
    };
    encode_to_file(&estimate, &path).expect("encode carbon estimate");
    let decoded: CarbonSequestrationEstimate =
        decode_from_file(&path).expect("decode carbon estimate");
    assert_eq!(estimate, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 15: Wildlife habitat assessment via vec slice ──────────────────────

#[test]
fn test_wildlife_habitat_assessment_vec_slice() {
    let assessments = vec![
        WildlifeHabitatAssessment {
            assessment_id: 40001,
            species_of_concern: "Northern Spotted Owl".into(),
            habitat_type: "Old-growth conifer forest".into(),
            quality: HabitatQuality::Good,
            patch_size_ha: 450.0,
            connectivity_index: 0.78,
            snag_density_per_ha: 12.3,
            canopy_closure_percent: 82.0,
        },
        WildlifeHabitatAssessment {
            assessment_id: 40002,
            species_of_concern: "Black-backed Woodpecker".into(),
            habitat_type: "Post-fire snag forest".into(),
            quality: HabitatQuality::Pristine,
            patch_size_ha: 180.0,
            connectivity_index: 0.45,
            snag_density_per_ha: 85.0,
            canopy_closure_percent: 5.0,
        },
    ];
    let bytes = encode_to_vec(&assessments).expect("encode habitat assessments");
    let (decoded, _): (Vec<WildlifeHabitatAssessment>, usize) =
        decode_from_slice(&bytes).expect("decode habitat assessments");
    assert_eq!(assessments, decoded);
}

// ── Test 16: Watershed protection zone via file ─────────────────────────────

#[test]
fn test_watershed_protection_zone_file() {
    let path = test_path("t16");
    let zone = WatershedProtectionZone {
        zone_id: 5500,
        watershed_name: "Upper Clearwater".into(),
        area_ha: 12_500.0,
        buffer_width_m: 30.0,
        stream_order: 4,
        is_municipal_supply: true,
        sediment_yield_tonnes_per_yr: 230.5,
        logging_restricted: true,
    };
    encode_to_file(&zone, &path).expect("encode watershed zone");
    let decoded: WatershedProtectionZone = decode_from_file(&path).expect("decode watershed zone");
    assert_eq!(zone, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 17: Logging road conditions via vec slice ──────────────────────────

#[test]
fn test_logging_road_conditions_vec_slice() {
    let roads = vec![
        LoggingRoad {
            road_id: 3001,
            road_name: "FS-2210 Spur A".into(),
            length_km: 8.5,
            surface_type: "Crushed aggregate".into(),
            condition: RoadCondition::Good,
            max_gross_vehicle_weight_kg: 36_000,
            drainage_structures_count: 14,
            seasonal_closure: false,
        },
        LoggingRoad {
            road_id: 3002,
            road_name: "FS-2210 Spur B".into(),
            length_km: 3.2,
            surface_type: "Native surface".into(),
            condition: RoadCondition::Poor,
            max_gross_vehicle_weight_kg: 18_000,
            drainage_structures_count: 5,
            seasonal_closure: true,
        },
        LoggingRoad {
            road_id: 3003,
            road_name: "FS-2211 Mainline".into(),
            length_km: 22.0,
            surface_type: "Asphalt".into(),
            condition: RoadCondition::Excellent,
            max_gross_vehicle_weight_kg: 45_000,
            drainage_structures_count: 38,
            seasonal_closure: false,
        },
    ];
    let bytes = encode_to_vec(&roads).expect("encode logging roads");
    let (decoded, _): (Vec<LoggingRoad>, usize) =
        decode_from_slice(&bytes).expect("decode logging roads");
    assert_eq!(roads, decoded);
}

// ── Test 18: Road condition enum coverage ───────────────────────────────────

#[test]
fn test_road_condition_all_variants_file() {
    let path = test_path("t18");
    let conditions = vec![
        RoadCondition::Excellent,
        RoadCondition::Good,
        RoadCondition::Fair,
        RoadCondition::Poor,
        RoadCondition::Impassable,
        RoadCondition::Decommissioned,
    ];
    encode_to_file(&conditions, &path).expect("encode road conditions");
    let decoded: Vec<RoadCondition> = decode_from_file(&path).expect("decode road conditions");
    assert_eq!(conditions, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 19: Remote sensing canopy analysis via file ────────────────────────

#[test]
fn test_canopy_analysis_file() {
    let path = test_path("t19");
    let analysis = CanopyAnalysis {
        tile_id: 770088,
        canopy_cover_percent: 73.5,
        canopy_height_mean_m: 24.8,
        canopy_height_max_m: 42.1,
        leaf_area_index: 4.2,
        ndvi_mean: 0.72,
        gap_fraction_percent: 18.3,
        sensor_platform: "Landsat 9 OLI-2".into(),
        resolution_m: 30.0,
    };
    encode_to_file(&analysis, &path).expect("encode canopy analysis");
    let decoded: CanopyAnalysis = decode_from_file(&path).expect("decode canopy analysis");
    assert_eq!(analysis, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Reforestation plan with multiple species via vec slice ─────────

#[test]
fn test_reforestation_plan_vec_slice() {
    let plan = ReforestationPlan {
        plan_id: 120001,
        site_name: "Burnt Creek Restoration Unit".into(),
        area_ha: 350.0,
        target_species: vec![
            TreeSpecies {
                common_name: "Ponderosa Pine".into(),
                scientific_name: "Pinus ponderosa".into(),
                fire_resistance_rating: 8,
                is_conifer: true,
            },
            TreeSpecies {
                common_name: "Western Larch".into(),
                scientific_name: "Larix occidentalis".into(),
                fire_resistance_rating: 9,
                is_conifer: true,
            },
            TreeSpecies {
                common_name: "Quaking Aspen".into(),
                scientific_name: "Populus tremuloides".into(),
                fire_resistance_rating: 4,
                is_conifer: false,
            },
        ],
        seedlings_per_ha: 740,
        planting_year: 2027,
        site_prep_method: "Mechanical scarification with slash pile burning".into(),
        expected_survival_percent: 72.0,
        monitoring_years: 10,
    };
    let bytes = encode_to_vec(&plan).expect("encode reforestation plan");
    let (decoded, _): (ReforestationPlan, usize) =
        decode_from_slice(&bytes).expect("decode reforestation plan");
    assert_eq!(plan, decoded);
}

// ── Test 21: Harvest equipment log via file ─────────────────────────────────

#[test]
fn test_harvest_equipment_log_file() {
    let path = test_path("t21");
    let log_entry = HarvestEquipmentLog {
        equipment_id: 60044,
        equipment_type: "Feller Buncher".into(),
        serial_number: "CAT-552-2024-00891".into(),
        hours_operated: 4832.5,
        fuel_consumed_liters: 28_400.0,
        volume_harvested_m3: 15_600.0,
        status: EquipmentStatus::Operational,
        operator_name: "James Kowalski".into(),
        last_service_date: "2026-02-28".into(),
    };
    encode_to_file(&log_entry, &path).expect("encode equipment log");
    let decoded: HarvestEquipmentLog = decode_from_file(&path).expect("decode equipment log");
    assert_eq!(log_entry, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 22: Full incident scenario combining multiple domain types ─────────

#[test]
fn test_full_wildfire_incident_scenario_file() {
    let path = test_path("t22");

    // A composite struct representing a full incident snapshot
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct WildfireIncidentSnapshot {
        incident_name: String,
        incident_number: u64,
        risk: WildfireRiskIndex,
        fire_behavior: FireBehaviorModel,
        resources: Vec<FirefightingResource>,
        smoke_forecast: SmokeDispersionForecast,
        watershed_threatened: Vec<WatershedProtectionZone>,
        habitat_at_risk: Vec<WildlifeHabitatAssessment>,
    }

    let snapshot = WildfireIncidentSnapshot {
        incident_name: "Granite Peak Complex".into(),
        incident_number: 2026_03_15_001,
        risk: WildfireRiskIndex {
            zone_id: 9900,
            fuel_moisture: FuelMoistureReading {
                station_id: 801,
                one_hour_percent: 2.1,
                ten_hour_percent: 4.3,
                hundred_hour_percent: 7.8,
                thousand_hour_percent: 12.0,
                live_herbaceous_percent: 30.0,
                live_woody_percent: 55.0,
            },
            wind_speed_kmh: 56.0,
            wind_direction_deg: 200,
            topography: TopographyClass::Ridgeline,
            drought_index: 720.0,
            overall_risk_score: 97.8,
        },
        fire_behavior: FireBehaviorModel {
            model_id: 20260315002,
            fuel_model_code: "SH9".into(),
            rate_of_spread_m_per_min: 28.0,
            flame_length_m: 6.5,
            fireline_intensity_kw_per_m: 12_500.0,
            spotting_distance_km: 3.5,
            crown_fire_potential: true,
            predicted_area_ha: 1_200.0,
        },
        resources: vec![
            FirefightingResource {
                resource_id: 20001,
                resource_type: ResourceType::Hotshot,
                assigned_division: "Division Alpha".into(),
                status_available: false,
                lat: 44.5210,
                lon: -115.3400,
                personnel_count: 20,
            },
            FirefightingResource {
                resource_id: 20002,
                resource_type: ResourceType::AirTanker(747),
                assigned_division: "Air Ops".into(),
                status_available: true,
                lat: 43.5640,
                lon: -116.2200,
                personnel_count: 4,
            },
            FirefightingResource {
                resource_id: 20003,
                resource_type: ResourceType::CommandPost,
                assigned_division: "ICP".into(),
                status_available: true,
                lat: 44.4800,
                lon: -115.2900,
                personnel_count: 35,
            },
        ],
        smoke_forecast: SmokeDispersionForecast {
            forecast_id: 880001,
            source_fire_name: "Granite Peak Complex".into(),
            pm25_ug_per_m3: 320.0,
            pm10_ug_per_m3: 410.0,
            visibility_km: 0.8,
            plume_height_m: 5500.0,
            wind_transport_direction_deg: 190,
            affected_communities: vec!["Stanley".into(), "Lowman".into(), "Garden Valley".into()],
            aqi_category: "Hazardous".into(),
        },
        watershed_threatened: vec![WatershedProtectionZone {
            zone_id: 6600,
            watershed_name: "South Fork Payette".into(),
            area_ha: 18_000.0,
            buffer_width_m: 50.0,
            stream_order: 5,
            is_municipal_supply: true,
            sediment_yield_tonnes_per_yr: 480.0,
            logging_restricted: true,
        }],
        habitat_at_risk: vec![WildlifeHabitatAssessment {
            assessment_id: 50001,
            species_of_concern: "Bull Trout".into(),
            habitat_type: "Cold-water stream with large woody debris".into(),
            quality: HabitatQuality::Good,
            patch_size_ha: 85.0,
            connectivity_index: 0.92,
            snag_density_per_ha: 0.0,
            canopy_closure_percent: 65.0,
        }],
    };

    encode_to_file(&snapshot, &path).expect("encode incident snapshot");
    let decoded: WildfireIncidentSnapshot =
        decode_from_file(&path).expect("decode incident snapshot");
    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).ok();
}
