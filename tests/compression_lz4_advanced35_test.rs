//! Advanced LZ4 compression tests for the forestry management and ecological monitoring domain.
//!
//! Covers tree inventory (species, DBH, height, health), timber harvest plans,
//! wildfire risk assessments, carbon sequestration estimates, wildlife corridor mappings,
//! stream buffer zones, soil erosion classifications, prescribed burn parameters,
//! invasive species monitoring, LiDAR canopy height models, seed bank inventories,
//! and forest regeneration tracking.

#![cfg(feature = "compression-lz4")]
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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum TreeHealthStatus {
    Vigorous,
    Stressed,
    Declining,
    Moribund,
    Dead { year_of_death: u16 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GpsCoord {
    latitude: f64,
    longitude: f64,
    elevation_m: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TreeRecord {
    tag_id: u64,
    species_code: String,
    common_name: String,
    dbh_cm: f64,
    height_m: f64,
    crown_class: u8,
    health: TreeHealthStatus,
    location: GpsCoord,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum HarvestMethod {
    ClearCut,
    SelectiveCut { min_dbh_cm: f64 },
    ShelterWood { retention_pct: u8 },
    SeedTree { trees_per_hectare: u32 },
    GroupSelection { opening_diameter_m: f64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TimberHarvestPlan {
    plan_id: u64,
    compartment: String,
    area_hectares: f64,
    method: HarvestMethod,
    estimated_volume_m3: f64,
    species_targets: Vec<String>,
    road_access_km: f64,
    seasonal_restriction: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FireRiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
    Extreme,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WildfireRiskAssessment {
    assessment_id: u32,
    zone_name: String,
    fuel_load_tonnes_per_ha: f64,
    slope_pct: f64,
    aspect_degrees: u16,
    risk_level: FireRiskLevel,
    days_since_rain: u32,
    wind_speed_kmh: f64,
    relative_humidity_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CarbonPool {
    pool_name: String,
    carbon_tonnes_per_ha: f64,
    uncertainty_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CarbonSequestrationEstimate {
    plot_id: u64,
    forest_type: String,
    stand_age_years: u32,
    pools: Vec<CarbonPool>,
    annual_increment_tonnes: f64,
    measurement_year: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CorridorType {
    Riparian { width_m: f64 },
    Ridge,
    Upland,
    Wetland,
    SteppingStone { patch_area_ha: f64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WildlifeCorridor {
    corridor_id: u64,
    name: String,
    corridor_type: CorridorType,
    length_km: f64,
    target_species: Vec<String>,
    connectivity_index: f64,
    nodes: Vec<GpsCoord>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BufferCategory {
    PerennialStream { order: u8 },
    IntermittentStream,
    Wetland,
    Lake,
    Spring,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StreamBufferZone {
    zone_id: u32,
    category: BufferCategory,
    required_width_m: f64,
    actual_width_m: f64,
    vegetation_cover_pct: u8,
    length_m: f64,
    compliance: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ErosionClass {
    None,
    Slight,
    Moderate,
    Severe,
    VerySevere,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ErosionType {
    Sheet,
    Rill,
    Gully { depth_m: f64 },
    Tunnel,
    Streambank,
    Mass { volume_m3: f64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SoilErosionRecord {
    site_id: u32,
    erosion_class: ErosionClass,
    erosion_type: ErosionType,
    slope_pct: f64,
    soil_texture: String,
    ground_cover_pct: u8,
    estimated_loss_tonnes_per_ha_yr: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PrescribedBurnParams {
    burn_id: u64,
    unit_name: String,
    area_hectares: f64,
    target_flame_length_m: f64,
    target_rate_of_spread_m_per_min: f64,
    min_relative_humidity: u8,
    max_relative_humidity: u8,
    min_wind_speed_kmh: f64,
    max_wind_speed_kmh: f64,
    acceptable_wind_directions: Vec<String>,
    fuel_moisture_pct: f64,
    ignition_pattern: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum InvasiveStatus {
    Detected,
    Established,
    Spreading,
    Contained,
    Eradicated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TreatmentMethod {
    Manual,
    Mechanical,
    Chemical { herbicide: String },
    Biological { agent: String },
    Integrated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct InvasiveSpeciesRecord {
    record_id: u64,
    species_name: String,
    common_name: String,
    status: InvasiveStatus,
    coverage_hectares: f64,
    density_per_m2: f64,
    treatment: TreatmentMethod,
    location: GpsCoord,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LidarCanopyCell {
    row: u32,
    col: u32,
    max_height_m: f64,
    mean_height_m: f64,
    canopy_cover_pct: f64,
    point_density: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LidarCanopyModel {
    model_id: u64,
    acquisition_date: String,
    resolution_m: f64,
    rows: u32,
    cols: u32,
    cells: Vec<LidarCanopyCell>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ViabilityClass {
    High,
    Medium,
    Low,
    Nonviable,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeedBankEntry {
    species: String,
    seeds_per_m2: f64,
    viability: ViabilityClass,
    depth_cm: f64,
    dormancy_type: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeedBankInventory {
    inventory_id: u32,
    plot_id: u64,
    sample_date: String,
    soil_type: String,
    entries: Vec<SeedBankEntry>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RegenerationOrigin {
    NaturalSeed,
    Sprout,
    Planted { nursery: String },
    DirectSeeding,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RegenerationSurvey {
    survey_id: u64,
    stand_id: String,
    years_since_disturbance: u8,
    seedlings_per_hectare: u32,
    dominant_species: String,
    origin: RegenerationOrigin,
    browsing_damage_pct: u8,
    competing_vegetation_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SnagDecayClass {
    Class1RecentlyDead,
    Class2LooseBark,
    Class3CleanBole,
    Class4BrokenTop,
    Class5Stump,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SnagRecord {
    tag_id: u32,
    species: String,
    dbh_cm: f64,
    height_m: f64,
    decay_class: SnagDecayClass,
    cavity_count: u8,
    wildlife_use: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WaterQualityRating {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WatershedMonitoring {
    station_id: u32,
    watershed_name: String,
    drainage_area_km2: f64,
    discharge_m3_per_s: f64,
    turbidity_ntu: f64,
    ph: f64,
    dissolved_oxygen_mg_per_l: f64,
    water_quality: WaterQualityRating,
    temperature_celsius: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CoarseWoodyDebris {
    transect_id: u32,
    diameter_cm: f64,
    length_m: f64,
    decay_class: u8,
    species: String,
    volume_m3: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeadwoodSurvey {
    survey_id: u64,
    plot_id: String,
    snags: Vec<SnagRecord>,
    logs: Vec<CoarseWoodyDebris>,
    total_volume_m3_per_ha: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ForestHealthSurvey {
    survey_id: u64,
    compartment: String,
    pest_species: Vec<String>,
    disease_agents: Vec<String>,
    defoliation_pct: u8,
    mortality_pct: u8,
    crown_dieback_pct: u8,
    affected_area_ha: f64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T) {
    let encoded = encode_to_vec(val).expect("encode value");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress lz4");
    let decompressed = decompress(&compressed).expect("decompress lz4");
    let (decoded, _): (T, usize) = decode_from_slice(&decompressed).expect("decode from slice");
    assert_eq!(*val, decoded);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_tree_record_lz4_roundtrip() {
    let tree = TreeRecord {
        tag_id: 100_421,
        species_code: String::from("PSME"),
        common_name: String::from("Douglas-fir"),
        dbh_cm: 54.6,
        height_m: 38.2,
        crown_class: 2,
        health: TreeHealthStatus::Vigorous,
        location: GpsCoord {
            latitude: 44.0582,
            longitude: -121.3153,
            elevation_m: 1340.0,
        },
    };
    roundtrip(&tree);
}

#[test]
fn test_dead_tree_with_year_of_death() {
    let tree = TreeRecord {
        tag_id: 200_910,
        species_code: String::from("ABCO"),
        common_name: String::from("White fir"),
        dbh_cm: 42.1,
        height_m: 22.0,
        crown_class: 4,
        health: TreeHealthStatus::Dead {
            year_of_death: 2023,
        },
        location: GpsCoord {
            latitude: 39.3175,
            longitude: -120.1942,
            elevation_m: 1920.5,
        },
    };
    roundtrip(&tree);
}

#[test]
fn test_timber_harvest_plan_selective() {
    let plan = TimberHarvestPlan {
        plan_id: 55001,
        compartment: String::from("NW-14B"),
        area_hectares: 32.7,
        method: HarvestMethod::SelectiveCut { min_dbh_cm: 40.0 },
        estimated_volume_m3: 1850.0,
        species_targets: vec![String::from("Douglas-fir"), String::from("Ponderosa pine")],
        road_access_km: 3.2,
        seasonal_restriction: true,
    };
    roundtrip(&plan);
}

#[test]
fn test_timber_harvest_plan_shelterwood() {
    let plan = TimberHarvestPlan {
        plan_id: 55002,
        compartment: String::from("SE-07A"),
        area_hectares: 18.4,
        method: HarvestMethod::ShelterWood { retention_pct: 30 },
        estimated_volume_m3: 920.0,
        species_targets: vec![String::from("Western red cedar")],
        road_access_km: 1.8,
        seasonal_restriction: false,
    };
    roundtrip(&plan);
}

#[test]
fn test_wildfire_risk_extreme() {
    let assessment = WildfireRiskAssessment {
        assessment_id: 7710,
        zone_name: String::from("Eagle Creek Ridge"),
        fuel_load_tonnes_per_ha: 48.3,
        slope_pct: 55.0,
        aspect_degrees: 225,
        risk_level: FireRiskLevel::Extreme,
        days_since_rain: 45,
        wind_speed_kmh: 35.0,
        relative_humidity_pct: 12,
    };
    roundtrip(&assessment);
}

#[test]
fn test_carbon_sequestration_multiple_pools() {
    let estimate = CarbonSequestrationEstimate {
        plot_id: 300_112,
        forest_type: String::from("Mixed conifer"),
        stand_age_years: 85,
        pools: vec![
            CarbonPool {
                pool_name: String::from("Aboveground live"),
                carbon_tonnes_per_ha: 142.3,
                uncertainty_pct: 8.5,
            },
            CarbonPool {
                pool_name: String::from("Belowground live"),
                carbon_tonnes_per_ha: 35.6,
                uncertainty_pct: 15.0,
            },
            CarbonPool {
                pool_name: String::from("Dead wood"),
                carbon_tonnes_per_ha: 18.9,
                uncertainty_pct: 22.0,
            },
            CarbonPool {
                pool_name: String::from("Litter"),
                carbon_tonnes_per_ha: 12.1,
                uncertainty_pct: 18.0,
            },
            CarbonPool {
                pool_name: String::from("Soil organic"),
                carbon_tonnes_per_ha: 95.0,
                uncertainty_pct: 25.0,
            },
        ],
        annual_increment_tonnes: 4.2,
        measurement_year: 2025,
    };
    roundtrip(&estimate);
}

#[test]
fn test_wildlife_corridor_riparian() {
    let corridor = WildlifeCorridor {
        corridor_id: 40001,
        name: String::from("Elk Creek Riparian Corridor"),
        corridor_type: CorridorType::Riparian { width_m: 100.0 },
        length_km: 12.4,
        target_species: vec![
            String::from("Roosevelt elk"),
            String::from("Northern spotted owl"),
            String::from("Pacific giant salamander"),
        ],
        connectivity_index: 0.87,
        nodes: vec![
            GpsCoord {
                latitude: 43.85,
                longitude: -122.10,
                elevation_m: 450.0,
            },
            GpsCoord {
                latitude: 43.90,
                longitude: -122.08,
                elevation_m: 520.0,
            },
            GpsCoord {
                latitude: 43.95,
                longitude: -122.05,
                elevation_m: 610.0,
            },
        ],
    };
    roundtrip(&corridor);
}

#[test]
fn test_wildlife_corridor_stepping_stone() {
    let corridor = WildlifeCorridor {
        corridor_id: 40002,
        name: String::from("Cascade Meadow Stepping Stones"),
        corridor_type: CorridorType::SteppingStone { patch_area_ha: 2.5 },
        length_km: 8.7,
        target_species: vec![
            String::from("Western toad"),
            String::from("Oregon spotted frog"),
        ],
        connectivity_index: 0.54,
        nodes: vec![
            GpsCoord {
                latitude: 44.12,
                longitude: -121.80,
                elevation_m: 1100.0,
            },
            GpsCoord {
                latitude: 44.15,
                longitude: -121.78,
                elevation_m: 1150.0,
            },
        ],
    };
    roundtrip(&corridor);
}

#[test]
fn test_stream_buffer_zone_perennial() {
    let zone = StreamBufferZone {
        zone_id: 891,
        category: BufferCategory::PerennialStream { order: 3 },
        required_width_m: 30.0,
        actual_width_m: 35.5,
        vegetation_cover_pct: 92,
        length_m: 1240.0,
        compliance: true,
    };
    roundtrip(&zone);
}

#[test]
fn test_stream_buffer_zone_noncompliant() {
    let zone = StreamBufferZone {
        zone_id: 892,
        category: BufferCategory::IntermittentStream,
        required_width_m: 15.0,
        actual_width_m: 8.0,
        vegetation_cover_pct: 41,
        length_m: 670.0,
        compliance: false,
    };
    roundtrip(&zone);
}

#[test]
fn test_soil_erosion_gully() {
    let record = SoilErosionRecord {
        site_id: 5501,
        erosion_class: ErosionClass::Severe,
        erosion_type: ErosionType::Gully { depth_m: 2.3 },
        slope_pct: 38.0,
        soil_texture: String::from("Sandy loam"),
        ground_cover_pct: 15,
        estimated_loss_tonnes_per_ha_yr: 24.7,
    };
    roundtrip(&record);
}

#[test]
fn test_soil_erosion_mass_movement() {
    let record = SoilErosionRecord {
        site_id: 5502,
        erosion_class: ErosionClass::VerySevere,
        erosion_type: ErosionType::Mass { volume_m3: 450.0 },
        slope_pct: 72.0,
        soil_texture: String::from("Silty clay"),
        ground_cover_pct: 5,
        estimated_loss_tonnes_per_ha_yr: 110.0,
    };
    roundtrip(&record);
}

#[test]
fn test_prescribed_burn_parameters() {
    let burn = PrescribedBurnParams {
        burn_id: 88100,
        unit_name: String::from("Ponderosa Flat Unit 3"),
        area_hectares: 45.0,
        target_flame_length_m: 0.6,
        target_rate_of_spread_m_per_min: 2.5,
        min_relative_humidity: 25,
        max_relative_humidity: 55,
        min_wind_speed_kmh: 5.0,
        max_wind_speed_kmh: 20.0,
        acceptable_wind_directions: vec![String::from("SW"), String::from("W"), String::from("NW")],
        fuel_moisture_pct: 8.0,
        ignition_pattern: String::from("Strip head fire"),
    };
    roundtrip(&burn);
}

#[test]
fn test_invasive_species_chemical_treatment() {
    let record = InvasiveSpeciesRecord {
        record_id: 60001,
        species_name: String::from("Cytisus scoparius"),
        common_name: String::from("Scotch broom"),
        status: InvasiveStatus::Spreading,
        coverage_hectares: 12.3,
        density_per_m2: 4.7,
        treatment: TreatmentMethod::Chemical {
            herbicide: String::from("Triclopyr"),
        },
        location: GpsCoord {
            latitude: 44.92,
            longitude: -123.01,
            elevation_m: 280.0,
        },
    };
    roundtrip(&record);
}

#[test]
fn test_invasive_species_biological_control() {
    let record = InvasiveSpeciesRecord {
        record_id: 60002,
        species_name: String::from("Lythrum salicaria"),
        common_name: String::from("Purple loosestrife"),
        status: InvasiveStatus::Contained,
        coverage_hectares: 0.8,
        density_per_m2: 12.0,
        treatment: TreatmentMethod::Biological {
            agent: String::from("Galerucella calmariensis"),
        },
        location: GpsCoord {
            latitude: 45.52,
            longitude: -122.68,
            elevation_m: 15.0,
        },
    };
    roundtrip(&record);
}

#[test]
fn test_lidar_canopy_height_model() {
    let model = LidarCanopyModel {
        model_id: 9001,
        acquisition_date: String::from("2025-07-15"),
        resolution_m: 1.0,
        rows: 3,
        cols: 3,
        cells: vec![
            LidarCanopyCell {
                row: 0,
                col: 0,
                max_height_m: 42.1,
                mean_height_m: 35.2,
                canopy_cover_pct: 95.0,
                point_density: 12.3,
            },
            LidarCanopyCell {
                row: 0,
                col: 1,
                max_height_m: 38.5,
                mean_height_m: 30.1,
                canopy_cover_pct: 88.0,
                point_density: 11.8,
            },
            LidarCanopyCell {
                row: 0,
                col: 2,
                max_height_m: 5.2,
                mean_height_m: 3.1,
                canopy_cover_pct: 22.0,
                point_density: 8.5,
            },
            LidarCanopyCell {
                row: 1,
                col: 0,
                max_height_m: 45.0,
                mean_height_m: 37.8,
                canopy_cover_pct: 98.0,
                point_density: 14.1,
            },
            LidarCanopyCell {
                row: 1,
                col: 1,
                max_height_m: 0.0,
                mean_height_m: 0.0,
                canopy_cover_pct: 0.0,
                point_density: 6.2,
            },
            LidarCanopyCell {
                row: 1,
                col: 2,
                max_height_m: 28.9,
                mean_height_m: 22.4,
                canopy_cover_pct: 75.0,
                point_density: 10.0,
            },
            LidarCanopyCell {
                row: 2,
                col: 0,
                max_height_m: 40.3,
                mean_height_m: 34.0,
                canopy_cover_pct: 91.0,
                point_density: 13.0,
            },
            LidarCanopyCell {
                row: 2,
                col: 1,
                max_height_m: 36.7,
                mean_height_m: 29.5,
                canopy_cover_pct: 85.0,
                point_density: 11.2,
            },
            LidarCanopyCell {
                row: 2,
                col: 2,
                max_height_m: 41.8,
                mean_height_m: 36.0,
                canopy_cover_pct: 93.0,
                point_density: 12.8,
            },
        ],
    };
    roundtrip(&model);
}

#[test]
fn test_seed_bank_inventory() {
    let inventory = SeedBankInventory {
        inventory_id: 770,
        plot_id: 300_112,
        sample_date: String::from("2025-09-20"),
        soil_type: String::from("Volcanic ash-derived andisol"),
        entries: vec![
            SeedBankEntry {
                species: String::from("Ceanothus velutinus"),
                seeds_per_m2: 3200.0,
                viability: ViabilityClass::High,
                depth_cm: 5.0,
                dormancy_type: String::from("Physical - hard seed coat"),
            },
            SeedBankEntry {
                species: String::from("Arctostaphylos patula"),
                seeds_per_m2: 850.0,
                viability: ViabilityClass::Medium,
                depth_cm: 3.0,
                dormancy_type: String::from("Physical - heat activated"),
            },
            SeedBankEntry {
                species: String::from("Pseudotsuga menziesii"),
                seeds_per_m2: 45.0,
                viability: ViabilityClass::Low,
                depth_cm: 1.5,
                dormancy_type: String::from("None - recalcitrant"),
            },
        ],
    };
    roundtrip(&inventory);
}

#[test]
fn test_regeneration_survey_natural() {
    let survey = RegenerationSurvey {
        survey_id: 19001,
        stand_id: String::from("T14S-R8E-S22"),
        years_since_disturbance: 5,
        seedlings_per_hectare: 1_200,
        dominant_species: String::from("Ponderosa pine"),
        origin: RegenerationOrigin::NaturalSeed,
        browsing_damage_pct: 18,
        competing_vegetation_pct: 45,
    };
    roundtrip(&survey);
}

#[test]
fn test_regeneration_survey_planted() {
    let survey = RegenerationSurvey {
        survey_id: 19002,
        stand_id: String::from("T12S-R6E-S08"),
        years_since_disturbance: 2,
        seedlings_per_hectare: 900,
        dominant_species: String::from("Douglas-fir"),
        origin: RegenerationOrigin::Planted {
            nursery: String::from("Dorena Genetic Resource Center"),
        },
        browsing_damage_pct: 8,
        competing_vegetation_pct: 60,
    };
    roundtrip(&survey);
}

#[test]
fn test_deadwood_survey_complex() {
    let survey = DeadwoodSurvey {
        survey_id: 25001,
        plot_id: String::from("FIA-OR-41035"),
        snags: vec![
            SnagRecord {
                tag_id: 1001,
                species: String::from("Grand fir"),
                dbh_cm: 48.0,
                height_m: 18.5,
                decay_class: SnagDecayClass::Class3CleanBole,
                cavity_count: 3,
                wildlife_use: vec![
                    String::from("Pileated woodpecker"),
                    String::from("White-breasted nuthatch"),
                ],
            },
            SnagRecord {
                tag_id: 1002,
                species: String::from("Ponderosa pine"),
                dbh_cm: 62.0,
                height_m: 8.0,
                decay_class: SnagDecayClass::Class4BrokenTop,
                cavity_count: 5,
                wildlife_use: vec![
                    String::from("American kestrel"),
                    String::from("Western bluebird"),
                    String::from("Little brown bat"),
                ],
            },
        ],
        logs: vec![
            CoarseWoodyDebris {
                transect_id: 1,
                diameter_cm: 35.0,
                length_m: 12.0,
                decay_class: 2,
                species: String::from("Douglas-fir"),
                volume_m3: 1.15,
            },
            CoarseWoodyDebris {
                transect_id: 1,
                diameter_cm: 55.0,
                length_m: 8.5,
                decay_class: 4,
                species: String::from("Western red cedar"),
                volume_m3: 2.02,
            },
        ],
        total_volume_m3_per_ha: 38.5,
    };
    roundtrip(&survey);
}

#[test]
fn test_forest_health_survey_pest_outbreak() {
    let survey = ForestHealthSurvey {
        survey_id: 33001,
        compartment: String::from("Blue Mountain Unit 7"),
        pest_species: vec![
            String::from("Dendroctonus ponderosae"),
            String::from("Ips pini"),
        ],
        disease_agents: vec![String::from("Armillaria ostoyae")],
        defoliation_pct: 35,
        mortality_pct: 12,
        crown_dieback_pct: 28,
        affected_area_ha: 156.0,
    };
    roundtrip(&survey);
}

#[test]
fn test_watershed_monitoring_station() {
    let stations = vec![
        WatershedMonitoring {
            station_id: 14210,
            watershed_name: String::from("McKenzie River - Upper"),
            drainage_area_km2: 345.0,
            discharge_m3_per_s: 28.4,
            turbidity_ntu: 2.1,
            ph: 7.2,
            dissolved_oxygen_mg_per_l: 10.8,
            water_quality: WaterQualityRating::Excellent,
            temperature_celsius: 8.5,
        },
        WatershedMonitoring {
            station_id: 14211,
            watershed_name: String::from("Willamette - Coast Fork"),
            drainage_area_km2: 680.0,
            discharge_m3_per_s: 42.1,
            turbidity_ntu: 18.5,
            ph: 6.8,
            dissolved_oxygen_mg_per_l: 7.2,
            water_quality: WaterQualityRating::Fair,
            temperature_celsius: 15.3,
        },
    ];
    roundtrip(&stations);
}
