//! Advanced file I/O tests for OxiCode — domain: urban planning and smart city infrastructure

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ZoningClassification {
    Residential,
    Commercial,
    Industrial,
    MixedUse,
    Agricultural,
    OpenSpace,
    Institutional,
    TransitOriented { radius_meters: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrafficFlowLevel {
    FreeFlow,
    ReasonablyFree,
    StableFlow,
    ApproachingUnstable,
    Unstable,
    ForcedBreakdown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransitMode {
    Bus,
    LightRail,
    Subway,
    CommuterRail,
    Ferry,
    CableCar,
    BusRapidTransit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PermitStatus {
    Applied,
    UnderReview,
    Approved,
    Denied,
    Expired,
    Revoked,
    ConditionalApproval { conditions_count: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum UtilityType {
    Electricity,
    NaturalGas,
    Water,
    Sewer,
    Stormwater,
    Telecom,
    DistrictHeating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmergencyResponseTier {
    Tier1Immediate,
    Tier2Rapid,
    Tier3Standard,
    Tier4Extended,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NoiseCategory {
    Quiet,
    Moderate,
    Loud,
    VeryLoud,
    ExtremelyLoud,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GreenSpaceType {
    Park,
    Garden,
    Playground,
    NatureReserve,
    GreenCorridor,
    RooftopGarden,
    UrbanFarm,
    Wetland,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoCoordinate {
    latitude_microdeg: i64,
    longitude_microdeg: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ZoningParcel {
    parcel_id: u64,
    classification: ZoningClassification,
    area_sq_meters: u32,
    max_building_height_cm: u32,
    floor_area_ratio_x100: u16,
    setback_front_cm: u16,
    setback_rear_cm: u16,
    center: GeoCoordinate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficSegment {
    segment_id: u64,
    road_name: String,
    flow_level: TrafficFlowLevel,
    avg_speed_kmh_x10: u16,
    vehicle_count_per_hour: u32,
    pedestrian_count_per_hour: u16,
    cyclist_count_per_hour: u16,
    lane_count: u8,
    has_bike_lane: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransitStop {
    stop_id: u32,
    name: String,
    coordinate: GeoCoordinate,
    modes: Vec<TransitMode>,
    daily_ridership: u32,
    wheelchair_accessible: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransitRoute {
    route_id: u32,
    route_name: String,
    mode: TransitMode,
    stops: Vec<TransitStop>,
    headway_seconds: u16,
    operating_hours_start_min: u16,
    operating_hours_end_min: u16,
    total_length_meters: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BuildingPermit {
    permit_number: u64,
    applicant_name: String,
    project_description: String,
    status: PermitStatus,
    parcel_id: u64,
    estimated_cost_cents: u64,
    total_floor_area_sq_m: u32,
    num_stories: u8,
    is_demolition: bool,
    submission_day: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UtilityGridNode {
    node_id: u64,
    utility_type: UtilityType,
    coordinate: GeoCoordinate,
    capacity_units: u64,
    current_load_units: u64,
    connected_nodes: Vec<u64>,
    is_operational: bool,
    last_maintenance_day: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PopulationDensityCell {
    grid_x: u32,
    grid_y: u32,
    population: u32,
    area_sq_meters: u32,
    avg_household_size_x10: u16,
    median_age_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GreenSpaceAllocation {
    space_id: u32,
    name: String,
    space_type: GreenSpaceType,
    area_sq_meters: u32,
    tree_count: u32,
    has_water_feature: bool,
    annual_visitors: u32,
    maintenance_budget_cents: u64,
    coordinate: GeoCoordinate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NoiseMeasurement {
    sensor_id: u32,
    coordinate: GeoCoordinate,
    decibel_x10: u16,
    category: NoiseCategory,
    timestamp_epoch_secs: u64,
    source_description: String,
    duration_seconds: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParkingStructure {
    structure_id: u32,
    name: String,
    total_spaces: u16,
    occupied_spaces: u16,
    ev_charging_spaces: u16,
    handicap_spaces: u16,
    floors: u8,
    hourly_rate_cents: u32,
    coordinate: GeoCoordinate,
    has_bicycle_parking: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyResponseZone {
    zone_id: u32,
    zone_name: String,
    tier: EmergencyResponseTier,
    population_covered: u32,
    fire_stations: Vec<u32>,
    hospital_ids: Vec<u32>,
    avg_response_time_seconds: u16,
    area_sq_km_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartLightPole {
    pole_id: u64,
    coordinate: GeoCoordinate,
    brightness_percent: u8,
    has_camera: bool,
    has_air_quality_sensor: bool,
    has_wifi_hotspot: bool,
    energy_consumption_wh: u32,
    last_fault_day: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterInfrastructureNode {
    node_id: u64,
    node_type: WaterNodeType,
    pressure_kpa_x10: u32,
    flow_rate_liters_per_min_x10: u32,
    pipe_diameter_mm: u16,
    material: PipeMaterial,
    installation_year: u16,
    coordinate: GeoCoordinate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaterNodeType {
    Reservoir,
    PumpStation,
    TreatmentPlant,
    DistributionMain,
    ServiceConnection,
    FireHydrant,
    PressureReducingValve,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PipeMaterial {
    CastIron,
    DuctileIron,
    Pvc,
    Hdpe,
    Concrete,
    Steel,
    Copper,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CityBudgetLineItem {
    department: String,
    category: String,
    allocated_cents: u64,
    spent_cents: u64,
    fiscal_year: u16,
    is_capital: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartCityDashboard {
    city_name: String,
    total_population: u64,
    zoning_parcels: Vec<ZoningParcel>,
    transit_routes: Vec<TransitRoute>,
    green_spaces: Vec<GreenSpaceAllocation>,
    emergency_zones: Vec<EmergencyResponseZone>,
    budget_items: Vec<CityBudgetLineItem>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_zoning_parcel_roundtrip() {
    let parcel = ZoningParcel {
        parcel_id: 100_234,
        classification: ZoningClassification::TransitOriented { radius_meters: 800 },
        area_sq_meters: 5_000,
        max_building_height_cm: 4_500,
        floor_area_ratio_x100: 350,
        setback_front_cm: 300,
        setback_rear_cm: 600,
        center: GeoCoordinate {
            latitude_microdeg: 40_712_776,
            longitude_microdeg: -74_005_974,
        },
    };
    let encoded = encode_to_vec(&parcel).expect("encode zoning parcel");
    let (decoded, _): (ZoningParcel, _) =
        decode_from_slice(&encoded).expect("decode zoning parcel");
    assert_eq!(parcel, decoded);
}

#[test]
fn test_zoning_classification_all_variants_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_zoning_variants.bin");
    let variants: Vec<ZoningClassification> = vec![
        ZoningClassification::Residential,
        ZoningClassification::Commercial,
        ZoningClassification::Industrial,
        ZoningClassification::MixedUse,
        ZoningClassification::Agricultural,
        ZoningClassification::OpenSpace,
        ZoningClassification::Institutional,
        ZoningClassification::TransitOriented { radius_meters: 400 },
    ];
    encode_to_file(&variants, &path).expect("encode zoning variants to file");
    let decoded: Vec<ZoningClassification> =
        decode_from_file(&path).expect("decode zoning variants from file");
    assert_eq!(variants, decoded);
    std::fs::remove_file(&path).expect("cleanup zoning variants file");
}

#[test]
fn test_traffic_segment_roundtrip() {
    let segment = TrafficSegment {
        segment_id: 88_001,
        road_name: "Broadway Avenue".to_string(),
        flow_level: TrafficFlowLevel::ApproachingUnstable,
        avg_speed_kmh_x10: 285,
        vehicle_count_per_hour: 1_850,
        pedestrian_count_per_hour: 420,
        cyclist_count_per_hour: 95,
        lane_count: 4,
        has_bike_lane: true,
    };
    let encoded = encode_to_vec(&segment).expect("encode traffic segment");
    let (decoded, _): (TrafficSegment, _) =
        decode_from_slice(&encoded).expect("decode traffic segment");
    assert_eq!(segment, decoded);
}

#[test]
fn test_traffic_flow_levels_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_traffic_flows.bin");
    let levels: Vec<TrafficFlowLevel> = vec![
        TrafficFlowLevel::FreeFlow,
        TrafficFlowLevel::ReasonablyFree,
        TrafficFlowLevel::StableFlow,
        TrafficFlowLevel::ApproachingUnstable,
        TrafficFlowLevel::Unstable,
        TrafficFlowLevel::ForcedBreakdown,
    ];
    encode_to_file(&levels, &path).expect("encode traffic flow levels to file");
    let decoded: Vec<TrafficFlowLevel> =
        decode_from_file(&path).expect("decode traffic flow levels from file");
    assert_eq!(levels, decoded);
    std::fs::remove_file(&path).expect("cleanup traffic flow levels file");
}

#[test]
fn test_transit_route_with_stops_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_transit_route.bin");
    let route = TransitRoute {
        route_id: 7,
        route_name: "Green Line Express".to_string(),
        mode: TransitMode::LightRail,
        stops: vec![
            TransitStop {
                stop_id: 101,
                name: "Central Station".to_string(),
                coordinate: GeoCoordinate {
                    latitude_microdeg: 48_856_614,
                    longitude_microdeg: 2_352_222,
                },
                modes: vec![TransitMode::LightRail, TransitMode::Subway],
                daily_ridership: 45_000,
                wheelchair_accessible: true,
            },
            TransitStop {
                stop_id: 102,
                name: "Innovation Park".to_string(),
                coordinate: GeoCoordinate {
                    latitude_microdeg: 48_860_000,
                    longitude_microdeg: 2_360_000,
                },
                modes: vec![TransitMode::LightRail],
                daily_ridership: 12_000,
                wheelchair_accessible: true,
            },
            TransitStop {
                stop_id: 103,
                name: "Riverside Terminal".to_string(),
                coordinate: GeoCoordinate {
                    latitude_microdeg: 48_865_000,
                    longitude_microdeg: 2_370_000,
                },
                modes: vec![TransitMode::LightRail, TransitMode::Ferry],
                daily_ridership: 8_500,
                wheelchair_accessible: false,
            },
        ],
        headway_seconds: 300,
        operating_hours_start_min: 330,
        operating_hours_end_min: 1_440,
        total_length_meters: 14_200,
    };
    encode_to_file(&route, &path).expect("encode transit route to file");
    let decoded: TransitRoute = decode_from_file(&path).expect("decode transit route from file");
    assert_eq!(route, decoded);
    std::fs::remove_file(&path).expect("cleanup transit route file");
}

#[test]
fn test_building_permit_roundtrip() {
    let permit = BuildingPermit {
        permit_number: 2026_0315_0001,
        applicant_name: "Metro Development Corp".to_string(),
        project_description: "Mixed-use tower with ground floor retail and 200 residential units"
            .to_string(),
        status: PermitStatus::ConditionalApproval {
            conditions_count: 12,
        },
        parcel_id: 55_432,
        estimated_cost_cents: 85_000_000_00,
        total_floor_area_sq_m: 28_000,
        num_stories: 25,
        is_demolition: false,
        submission_day: 20260101,
    };
    let encoded = encode_to_vec(&permit).expect("encode building permit");
    let (decoded, _): (BuildingPermit, _) =
        decode_from_slice(&encoded).expect("decode building permit");
    assert_eq!(permit, decoded);
}

#[test]
fn test_utility_grid_network_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_utility_grid.bin");
    let nodes = vec![
        UtilityGridNode {
            node_id: 1,
            utility_type: UtilityType::Electricity,
            coordinate: GeoCoordinate {
                latitude_microdeg: 35_689_487,
                longitude_microdeg: 139_691_711,
            },
            capacity_units: 500_000,
            current_load_units: 320_000,
            connected_nodes: vec![2, 3, 5],
            is_operational: true,
            last_maintenance_day: 20260210,
        },
        UtilityGridNode {
            node_id: 2,
            utility_type: UtilityType::Electricity,
            coordinate: GeoCoordinate {
                latitude_microdeg: 35_690_000,
                longitude_microdeg: 139_700_000,
            },
            capacity_units: 200_000,
            current_load_units: 198_500,
            connected_nodes: vec![1, 4],
            is_operational: true,
            last_maintenance_day: 20260115,
        },
        UtilityGridNode {
            node_id: 3,
            utility_type: UtilityType::NaturalGas,
            coordinate: GeoCoordinate {
                latitude_microdeg: 35_685_000,
                longitude_microdeg: 139_695_000,
            },
            capacity_units: 100_000,
            current_load_units: 0,
            connected_nodes: vec![1],
            is_operational: false,
            last_maintenance_day: 20251220,
        },
    ];
    encode_to_file(&nodes, &path).expect("encode utility grid nodes to file");
    let decoded: Vec<UtilityGridNode> =
        decode_from_file(&path).expect("decode utility grid nodes from file");
    assert_eq!(nodes, decoded);
    std::fs::remove_file(&path).expect("cleanup utility grid file");
}

#[test]
fn test_population_density_grid_roundtrip() {
    let cells: Vec<PopulationDensityCell> = (0..16)
        .map(|i| PopulationDensityCell {
            grid_x: i % 4,
            grid_y: i / 4,
            population: 500 + i * 120,
            area_sq_meters: 250_000,
            avg_household_size_x10: 27 + (i % 5) as u16,
            median_age_x10: 340 + (i * 3) as u16,
        })
        .collect();
    let encoded = encode_to_vec(&cells).expect("encode population density grid");
    let (decoded, _): (Vec<PopulationDensityCell>, _) =
        decode_from_slice(&encoded).expect("decode population density grid");
    assert_eq!(cells, decoded);
}

#[test]
fn test_green_space_allocations_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_green_spaces.bin");
    let spaces = vec![
        GreenSpaceAllocation {
            space_id: 1,
            name: "Millennium Park".to_string(),
            space_type: GreenSpaceType::Park,
            area_sq_meters: 100_000,
            tree_count: 2_500,
            has_water_feature: true,
            annual_visitors: 12_000_000,
            maintenance_budget_cents: 5_500_000_00,
            coordinate: GeoCoordinate {
                latitude_microdeg: 41_882_702,
                longitude_microdeg: -87_622_554,
            },
        },
        GreenSpaceAllocation {
            space_id: 2,
            name: "Rooftop Garden Block 7".to_string(),
            space_type: GreenSpaceType::RooftopGarden,
            area_sq_meters: 800,
            tree_count: 15,
            has_water_feature: false,
            annual_visitors: 3_600,
            maintenance_budget_cents: 45_000_00,
            coordinate: GeoCoordinate {
                latitude_microdeg: 41_885_000,
                longitude_microdeg: -87_625_000,
            },
        },
        GreenSpaceAllocation {
            space_id: 3,
            name: "Riverfront Wetland Preserve".to_string(),
            space_type: GreenSpaceType::Wetland,
            area_sq_meters: 250_000,
            tree_count: 8_000,
            has_water_feature: true,
            annual_visitors: 450_000,
            maintenance_budget_cents: 2_200_000_00,
            coordinate: GeoCoordinate {
                latitude_microdeg: 41_878_000,
                longitude_microdeg: -87_630_000,
            },
        },
    ];
    encode_to_file(&spaces, &path).expect("encode green spaces to file");
    let decoded: Vec<GreenSpaceAllocation> =
        decode_from_file(&path).expect("decode green spaces from file");
    assert_eq!(spaces, decoded);
    std::fs::remove_file(&path).expect("cleanup green spaces file");
}

#[test]
fn test_noise_measurements_roundtrip() {
    let measurements = vec![
        NoiseMeasurement {
            sensor_id: 301,
            coordinate: GeoCoordinate {
                latitude_microdeg: 51_507_351,
                longitude_microdeg: -127_580,
            },
            decibel_x10: 750,
            category: NoiseCategory::Loud,
            timestamp_epoch_secs: 1_710_500_000,
            source_description: "Construction site adjacent to A-road".to_string(),
            duration_seconds: 28_800,
        },
        NoiseMeasurement {
            sensor_id: 302,
            coordinate: GeoCoordinate {
                latitude_microdeg: 51_510_000,
                longitude_microdeg: -130_000,
            },
            decibel_x10: 450,
            category: NoiseCategory::Quiet,
            timestamp_epoch_secs: 1_710_500_000,
            source_description: "Residential zone at night".to_string(),
            duration_seconds: 3_600,
        },
    ];
    let encoded = encode_to_vec(&measurements).expect("encode noise measurements");
    let (decoded, _): (Vec<NoiseMeasurement>, _) =
        decode_from_slice(&encoded).expect("decode noise measurements");
    assert_eq!(measurements, decoded);
}

#[test]
fn test_parking_structure_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_parking.bin");
    let parking = ParkingStructure {
        structure_id: 42,
        name: "Downtown Gateway Garage".to_string(),
        total_spaces: 1_200,
        occupied_spaces: 987,
        ev_charging_spaces: 60,
        handicap_spaces: 24,
        floors: 6,
        hourly_rate_cents: 450,
        coordinate: GeoCoordinate {
            latitude_microdeg: 37_774_929,
            longitude_microdeg: -122_419_416,
        },
        has_bicycle_parking: true,
    };
    encode_to_file(&parking, &path).expect("encode parking structure to file");
    let decoded: ParkingStructure =
        decode_from_file(&path).expect("decode parking structure from file");
    assert_eq!(parking, decoded);
    std::fs::remove_file(&path).expect("cleanup parking file");
}

#[test]
fn test_emergency_response_zones_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_emergency_zones.bin");
    let zones = vec![
        EmergencyResponseZone {
            zone_id: 1,
            zone_name: "Downtown Core".to_string(),
            tier: EmergencyResponseTier::Tier1Immediate,
            population_covered: 85_000,
            fire_stations: vec![1, 3, 7],
            hospital_ids: vec![101, 102],
            avg_response_time_seconds: 240,
            area_sq_km_x100: 450,
        },
        EmergencyResponseZone {
            zone_id: 2,
            zone_name: "Suburban West".to_string(),
            tier: EmergencyResponseTier::Tier3Standard,
            population_covered: 42_000,
            fire_stations: vec![12],
            hospital_ids: vec![105],
            avg_response_time_seconds: 540,
            area_sq_km_x100: 2_800,
        },
    ];
    encode_to_file(&zones, &path).expect("encode emergency response zones to file");
    let decoded: Vec<EmergencyResponseZone> =
        decode_from_file(&path).expect("decode emergency response zones from file");
    assert_eq!(zones, decoded);
    std::fs::remove_file(&path).expect("cleanup emergency zones file");
}

#[test]
fn test_smart_light_pole_with_optional_fault() {
    let poles = vec![
        SmartLightPole {
            pole_id: 5001,
            coordinate: GeoCoordinate {
                latitude_microdeg: 52_520_007,
                longitude_microdeg: 13_404_954,
            },
            brightness_percent: 80,
            has_camera: true,
            has_air_quality_sensor: true,
            has_wifi_hotspot: false,
            energy_consumption_wh: 1_200,
            last_fault_day: None,
        },
        SmartLightPole {
            pole_id: 5002,
            coordinate: GeoCoordinate {
                latitude_microdeg: 52_521_000,
                longitude_microdeg: 13_406_000,
            },
            brightness_percent: 0,
            has_camera: false,
            has_air_quality_sensor: false,
            has_wifi_hotspot: true,
            energy_consumption_wh: 0,
            last_fault_day: Some(20260312),
        },
    ];
    let encoded = encode_to_vec(&poles).expect("encode smart light poles");
    let (decoded, _): (Vec<SmartLightPole>, _) =
        decode_from_slice(&encoded).expect("decode smart light poles");
    assert_eq!(poles, decoded);
}

#[test]
fn test_water_infrastructure_network_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_water_infra.bin");
    let nodes = vec![
        WaterInfrastructureNode {
            node_id: 9001,
            node_type: WaterNodeType::Reservoir,
            pressure_kpa_x10: 5_500,
            flow_rate_liters_per_min_x10: 120_000,
            pipe_diameter_mm: 900,
            material: PipeMaterial::Steel,
            installation_year: 1985,
            coordinate: GeoCoordinate {
                latitude_microdeg: 34_052_234,
                longitude_microdeg: -118_243_685,
            },
        },
        WaterInfrastructureNode {
            node_id: 9002,
            node_type: WaterNodeType::PumpStation,
            pressure_kpa_x10: 4_200,
            flow_rate_liters_per_min_x10: 80_000,
            pipe_diameter_mm: 600,
            material: PipeMaterial::DuctileIron,
            installation_year: 2005,
            coordinate: GeoCoordinate {
                latitude_microdeg: 34_060_000,
                longitude_microdeg: -118_250_000,
            },
        },
        WaterInfrastructureNode {
            node_id: 9003,
            node_type: WaterNodeType::FireHydrant,
            pressure_kpa_x10: 3_800,
            flow_rate_liters_per_min_x10: 6_000,
            pipe_diameter_mm: 150,
            material: PipeMaterial::CastIron,
            installation_year: 1972,
            coordinate: GeoCoordinate {
                latitude_microdeg: 34_055_000,
                longitude_microdeg: -118_248_000,
            },
        },
    ];
    encode_to_file(&nodes, &path).expect("encode water infrastructure to file");
    let decoded: Vec<WaterInfrastructureNode> =
        decode_from_file(&path).expect("decode water infrastructure from file");
    assert_eq!(nodes, decoded);
    std::fs::remove_file(&path).expect("cleanup water infrastructure file");
}

#[test]
fn test_city_budget_line_items_roundtrip() {
    let items = vec![
        CityBudgetLineItem {
            department: "Transportation".to_string(),
            category: "Road Maintenance".to_string(),
            allocated_cents: 15_000_000_00,
            spent_cents: 12_345_678_90,
            fiscal_year: 2026,
            is_capital: false,
        },
        CityBudgetLineItem {
            department: "Parks and Recreation".to_string(),
            category: "New Park Construction".to_string(),
            allocated_cents: 8_500_000_00,
            spent_cents: 2_100_000_00,
            fiscal_year: 2026,
            is_capital: true,
        },
        CityBudgetLineItem {
            department: "Public Safety".to_string(),
            category: "Fire Station Equipment".to_string(),
            allocated_cents: 3_200_000_00,
            spent_cents: 3_199_500_00,
            fiscal_year: 2026,
            is_capital: true,
        },
    ];
    let encoded = encode_to_vec(&items).expect("encode budget items");
    let (decoded, _): (Vec<CityBudgetLineItem>, _) =
        decode_from_slice(&encoded).expect("decode budget items");
    assert_eq!(items, decoded);
}

#[test]
fn test_permit_status_all_variants_roundtrip() {
    let statuses: Vec<PermitStatus> = vec![
        PermitStatus::Applied,
        PermitStatus::UnderReview,
        PermitStatus::Approved,
        PermitStatus::Denied,
        PermitStatus::Expired,
        PermitStatus::Revoked,
        PermitStatus::ConditionalApproval {
            conditions_count: 5,
        },
    ];
    let encoded = encode_to_vec(&statuses).expect("encode permit statuses");
    let (decoded, _): (Vec<PermitStatus>, _) =
        decode_from_slice(&encoded).expect("decode permit statuses");
    assert_eq!(statuses, decoded);
}

#[test]
fn test_smart_city_dashboard_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_dashboard.bin");
    let dashboard = SmartCityDashboard {
        city_name: "Neo Metropolis".to_string(),
        total_population: 2_350_000,
        zoning_parcels: vec![ZoningParcel {
            parcel_id: 1,
            classification: ZoningClassification::Residential,
            area_sq_meters: 12_000,
            max_building_height_cm: 2_000,
            floor_area_ratio_x100: 150,
            setback_front_cm: 500,
            setback_rear_cm: 300,
            center: GeoCoordinate {
                latitude_microdeg: 59_329_323,
                longitude_microdeg: 18_068_581,
            },
        }],
        transit_routes: vec![TransitRoute {
            route_id: 1,
            route_name: "Blue Line".to_string(),
            mode: TransitMode::Subway,
            stops: vec![TransitStop {
                stop_id: 1,
                name: "City Hall".to_string(),
                coordinate: GeoCoordinate {
                    latitude_microdeg: 59_330_000,
                    longitude_microdeg: 18_070_000,
                },
                modes: vec![TransitMode::Subway, TransitMode::Bus],
                daily_ridership: 30_000,
                wheelchair_accessible: true,
            }],
            headway_seconds: 180,
            operating_hours_start_min: 300,
            operating_hours_end_min: 1_440,
            total_length_meters: 22_500,
        }],
        green_spaces: vec![GreenSpaceAllocation {
            space_id: 1,
            name: "Central Botanical Garden".to_string(),
            space_type: GreenSpaceType::Garden,
            area_sq_meters: 45_000,
            tree_count: 1_200,
            has_water_feature: true,
            annual_visitors: 800_000,
            maintenance_budget_cents: 1_800_000_00,
            coordinate: GeoCoordinate {
                latitude_microdeg: 59_331_000,
                longitude_microdeg: 18_065_000,
            },
        }],
        emergency_zones: vec![EmergencyResponseZone {
            zone_id: 1,
            zone_name: "District Alpha".to_string(),
            tier: EmergencyResponseTier::Tier1Immediate,
            population_covered: 120_000,
            fire_stations: vec![1, 2, 5],
            hospital_ids: vec![10, 11],
            avg_response_time_seconds: 210,
            area_sq_km_x100: 600,
        }],
        budget_items: vec![CityBudgetLineItem {
            department: "Infrastructure".to_string(),
            category: "Smart Grid Upgrade".to_string(),
            allocated_cents: 50_000_000_00,
            spent_cents: 18_750_000_00,
            fiscal_year: 2026,
            is_capital: true,
        }],
    };
    encode_to_file(&dashboard, &path).expect("encode smart city dashboard to file");
    let decoded: SmartCityDashboard =
        decode_from_file(&path).expect("decode smart city dashboard from file");
    assert_eq!(dashboard, decoded);
    std::fs::remove_file(&path).expect("cleanup dashboard file");
}

#[test]
fn test_multiple_parking_structures_roundtrip() {
    let structures: Vec<ParkingStructure> = (0u16..5)
        .map(|i| ParkingStructure {
            structure_id: 100 + i as u32,
            name: format!("Garage {}", (b'A' + i as u8) as char),
            total_spaces: 200 + i * 100,
            occupied_spaces: 150 + i * 50,
            ev_charging_spaces: 10 + i * 5,
            handicap_spaces: 4 + i,
            floors: 3 + (i as u8 % 4),
            hourly_rate_cents: 300 + i as u32 * 50,
            coordinate: GeoCoordinate {
                latitude_microdeg: 45_464_204 + (i as i64) * 1_000,
                longitude_microdeg: 9_189_982 + (i as i64) * 1_500,
            },
            has_bicycle_parking: i % 2 == 0,
        })
        .collect();
    let encoded = encode_to_vec(&structures).expect("encode parking structures");
    let (decoded, _): (Vec<ParkingStructure>, _) =
        decode_from_slice(&encoded).expect("decode parking structures");
    assert_eq!(structures, decoded);
}

#[test]
fn test_transit_mode_all_variants_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_transit_modes.bin");
    let modes: Vec<TransitMode> = vec![
        TransitMode::Bus,
        TransitMode::LightRail,
        TransitMode::Subway,
        TransitMode::CommuterRail,
        TransitMode::Ferry,
        TransitMode::CableCar,
        TransitMode::BusRapidTransit,
    ];
    encode_to_file(&modes, &path).expect("encode transit modes to file");
    let decoded: Vec<TransitMode> =
        decode_from_file(&path).expect("decode transit modes from file");
    assert_eq!(modes, decoded);
    std::fs::remove_file(&path).expect("cleanup transit modes file");
}

#[test]
fn test_empty_collections_roundtrip() {
    let dashboard = SmartCityDashboard {
        city_name: "Ghost Town".to_string(),
        total_population: 0,
        zoning_parcels: vec![],
        transit_routes: vec![],
        green_spaces: vec![],
        emergency_zones: vec![],
        budget_items: vec![],
    };
    let encoded = encode_to_vec(&dashboard).expect("encode empty dashboard");
    let (decoded, _): (SmartCityDashboard, _) =
        decode_from_slice(&encoded).expect("decode empty dashboard");
    assert_eq!(dashboard, decoded);
}

#[test]
fn test_large_population_density_grid_file() {
    let path = temp_dir().join("oxicode_test_urban_planning_large_pop_grid.bin");
    let cells: Vec<PopulationDensityCell> = (0..400)
        .map(|i| PopulationDensityCell {
            grid_x: i % 20,
            grid_y: i / 20,
            population: 100 + (i * 37) % 50_000,
            area_sq_meters: 62_500,
            avg_household_size_x10: 20 + (i % 15) as u16,
            median_age_x10: 200 + ((i * 7) % 400) as u16,
        })
        .collect();
    encode_to_file(&cells, &path).expect("encode large population grid to file");
    let decoded: Vec<PopulationDensityCell> =
        decode_from_file(&path).expect("decode large population grid from file");
    assert_eq!(cells, decoded);
    std::fs::remove_file(&path).expect("cleanup large population grid file");
}

#[test]
fn test_pipe_material_and_water_node_variants_roundtrip() {
    let nodes: Vec<WaterInfrastructureNode> = vec![
        WaterInfrastructureNode {
            node_id: 1,
            node_type: WaterNodeType::TreatmentPlant,
            pressure_kpa_x10: 6_000,
            flow_rate_liters_per_min_x10: 200_000,
            pipe_diameter_mm: 1_200,
            material: PipeMaterial::Concrete,
            installation_year: 1990,
            coordinate: GeoCoordinate {
                latitude_microdeg: 55_755_826,
                longitude_microdeg: 37_617_300,
            },
        },
        WaterInfrastructureNode {
            node_id: 2,
            node_type: WaterNodeType::PressureReducingValve,
            pressure_kpa_x10: 3_500,
            flow_rate_liters_per_min_x10: 15_000,
            pipe_diameter_mm: 200,
            material: PipeMaterial::Hdpe,
            installation_year: 2020,
            coordinate: GeoCoordinate {
                latitude_microdeg: 55_760_000,
                longitude_microdeg: 37_620_000,
            },
        },
        WaterInfrastructureNode {
            node_id: 3,
            node_type: WaterNodeType::ServiceConnection,
            pressure_kpa_x10: 2_800,
            flow_rate_liters_per_min_x10: 500,
            pipe_diameter_mm: 25,
            material: PipeMaterial::Copper,
            installation_year: 2018,
            coordinate: GeoCoordinate {
                latitude_microdeg: 55_758_000,
                longitude_microdeg: 37_622_000,
            },
        },
        WaterInfrastructureNode {
            node_id: 4,
            node_type: WaterNodeType::DistributionMain,
            pressure_kpa_x10: 4_000,
            flow_rate_liters_per_min_x10: 50_000,
            pipe_diameter_mm: 400,
            material: PipeMaterial::Pvc,
            installation_year: 2012,
            coordinate: GeoCoordinate {
                latitude_microdeg: 55_762_000,
                longitude_microdeg: 37_625_000,
            },
        },
    ];
    let encoded = encode_to_vec(&nodes).expect("encode water node variants");
    let (decoded, _): (Vec<WaterInfrastructureNode>, _) =
        decode_from_slice(&encoded).expect("decode water node variants");
    assert_eq!(nodes, decoded);
}
