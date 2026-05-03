//! Advanced file I/O tests for disaster response / emergency management domain

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

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("{}_{}", name, std::process::id()))
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum IncidentType {
    Earthquake,
    Flood,
    Wildfire,
    HurricaneTypoon,
    ChemicalSpill,
    Infrastructure,
    MassEvent,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ResponseStatus {
    Monitoring,
    Responding,
    Contained,
    Recovery,
    Closed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Location {
    lat_micro: i32,
    lon_micro: i32,
    radius_m: u32,
    description: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ResourceUnit {
    unit_id: u32,
    unit_type: String,
    personnel_count: u16,
    equipment_count: u16,
    available: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IncidentReport {
    incident_id: u64,
    incident_type: IncidentType,
    status: ResponseStatus,
    location: Location,
    severity: u8,
    affected_count: u32,
    reported_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ResponsePlan {
    plan_id: u64,
    incident_id: u64,
    resources: Vec<ResourceUnit>,
    evacuation_zones: Vec<Location>,
    estimated_duration_h: u32,
}

// Test 1: IncidentType::Earthquake to file
#[test]
fn test_incident_type_earthquake_file() {
    let path = tmp("test_disaster_001.bin");
    let value = IncidentType::Earthquake;
    encode_to_file(&value, &path).expect("Failed to encode Earthquake to file");
    let decoded: IncidentType =
        decode_from_file(&path).expect("Failed to decode Earthquake from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: IncidentType::Flood to file
#[test]
fn test_incident_type_flood_file() {
    let path = tmp("test_disaster_002.bin");
    let value = IncidentType::Flood;
    encode_to_file(&value, &path).expect("Failed to encode Flood to file");
    let decoded: IncidentType = decode_from_file(&path).expect("Failed to decode Flood from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: IncidentType::Wildfire to file
#[test]
fn test_incident_type_wildfire_file() {
    let path = tmp("test_disaster_003.bin");
    let value = IncidentType::Wildfire;
    encode_to_file(&value, &path).expect("Failed to encode Wildfire to file");
    let decoded: IncidentType =
        decode_from_file(&path).expect("Failed to decode Wildfire from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: IncidentType::HurricaneTypoon to file
#[test]
fn test_incident_type_hurricane_typoon_file() {
    let path = tmp("test_disaster_004.bin");
    let value = IncidentType::HurricaneTypoon;
    encode_to_file(&value, &path).expect("Failed to encode HurricaneTypoon to file");
    let decoded: IncidentType =
        decode_from_file(&path).expect("Failed to decode HurricaneTypoon from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: ResponseStatus::Monitoring to file
#[test]
fn test_response_status_monitoring_file() {
    let path = tmp("test_disaster_005.bin");
    let value = ResponseStatus::Monitoring;
    encode_to_file(&value, &path).expect("Failed to encode Monitoring to file");
    let decoded: ResponseStatus =
        decode_from_file(&path).expect("Failed to decode Monitoring from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: ResponseStatus::Responding to file
#[test]
fn test_response_status_responding_file() {
    let path = tmp("test_disaster_006.bin");
    let value = ResponseStatus::Responding;
    encode_to_file(&value, &path).expect("Failed to encode Responding to file");
    let decoded: ResponseStatus =
        decode_from_file(&path).expect("Failed to decode Responding from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 7: ResponseStatus::Contained to file
#[test]
fn test_response_status_contained_file() {
    let path = tmp("test_disaster_007.bin");
    let value = ResponseStatus::Contained;
    encode_to_file(&value, &path).expect("Failed to encode Contained to file");
    let decoded: ResponseStatus =
        decode_from_file(&path).expect("Failed to decode Contained from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: Location file roundtrip
#[test]
fn test_location_file_roundtrip() {
    let path = tmp("test_disaster_008.bin");
    let value = Location {
        lat_micro: 37_774_929,
        lon_micro: -122_419_416,
        radius_m: 5000,
        description: String::from("San Francisco downtown incident zone"),
    };
    encode_to_file(&value, &path).expect("Failed to encode Location to file");
    let decoded: Location = decode_from_file(&path).expect("Failed to decode Location from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 9: ResourceUnit file roundtrip - available unit
#[test]
fn test_resource_unit_available_file_roundtrip() {
    let path = tmp("test_disaster_009.bin");
    let value = ResourceUnit {
        unit_id: 101,
        unit_type: String::from("Fire Engine"),
        personnel_count: 6,
        equipment_count: 12,
        available: true,
    };
    encode_to_file(&value, &path).expect("Failed to encode available ResourceUnit to file");
    let decoded: ResourceUnit =
        decode_from_file(&path).expect("Failed to decode available ResourceUnit from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 10: ResourceUnit file roundtrip - unavailable unit
#[test]
fn test_resource_unit_unavailable_file_roundtrip() {
    let path = tmp("test_disaster_010.bin");
    let value = ResourceUnit {
        unit_id: 202,
        unit_type: String::from("Hazmat Team"),
        personnel_count: 8,
        equipment_count: 24,
        available: false,
    };
    encode_to_file(&value, &path).expect("Failed to encode unavailable ResourceUnit to file");
    let decoded: ResourceUnit =
        decode_from_file(&path).expect("Failed to decode unavailable ResourceUnit from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 11: IncidentReport file roundtrip
#[test]
fn test_incident_report_file_roundtrip() {
    let path = tmp("test_disaster_011.bin");
    let value = IncidentReport {
        incident_id: 9_001_u64,
        incident_type: IncidentType::ChemicalSpill,
        status: ResponseStatus::Responding,
        location: Location {
            lat_micro: 40_712_776,
            lon_micro: -74_005_974,
            radius_m: 800,
            description: String::from("Industrial port area zone B"),
        },
        severity: 7,
        affected_count: 320,
        reported_at: 1_700_000_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode IncidentReport to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode IncidentReport from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 12: ResponsePlan with empty resources
#[test]
fn test_response_plan_empty_resources_file() {
    let path = tmp("test_disaster_012.bin");
    let value = ResponsePlan {
        plan_id: 5001,
        incident_id: 9_001,
        resources: vec![],
        evacuation_zones: vec![],
        estimated_duration_h: 2,
    };
    encode_to_file(&value, &path).expect("Failed to encode empty ResponsePlan to file");
    let decoded: ResponsePlan =
        decode_from_file(&path).expect("Failed to decode empty ResponsePlan from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 13: ResponsePlan with 5 resource units
#[test]
fn test_response_plan_five_resources_file() {
    let path = tmp("test_disaster_013.bin");
    let resources: Vec<ResourceUnit> = (1..=5)
        .map(|i| ResourceUnit {
            unit_id: i,
            unit_type: format!("Unit Type {}", i),
            personnel_count: (i * 3) as u16,
            equipment_count: (i * 5) as u16,
            available: i % 2 == 0,
        })
        .collect();
    let value = ResponsePlan {
        plan_id: 5002,
        incident_id: 9_002,
        resources,
        evacuation_zones: vec![],
        estimated_duration_h: 12,
    };
    encode_to_file(&value, &path).expect("Failed to encode 5-resource ResponsePlan to file");
    let decoded: ResponsePlan =
        decode_from_file(&path).expect("Failed to decode 5-resource ResponsePlan from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 14: ResponsePlan with 3 evacuation zones
#[test]
fn test_response_plan_three_evacuation_zones_file() {
    let path = tmp("test_disaster_014.bin");
    let evacuation_zones = vec![
        Location {
            lat_micro: 34_052_235,
            lon_micro: -118_243_683,
            radius_m: 2000,
            description: String::from("Zone Alpha - residential"),
        },
        Location {
            lat_micro: 34_062_235,
            lon_micro: -118_253_683,
            radius_m: 1500,
            description: String::from("Zone Beta - commercial"),
        },
        Location {
            lat_micro: 34_072_235,
            lon_micro: -118_263_683,
            radius_m: 3000,
            description: String::from("Zone Gamma - industrial"),
        },
    ];
    let value = ResponsePlan {
        plan_id: 5003,
        incident_id: 9_003,
        resources: vec![],
        evacuation_zones,
        estimated_duration_h: 6,
    };
    encode_to_file(&value, &path).expect("Failed to encode 3-zone ResponsePlan to file");
    let decoded: ResponsePlan =
        decode_from_file(&path).expect("Failed to decode 3-zone ResponsePlan from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 15: Vec<IncidentReport> file roundtrip
#[test]
fn test_vec_incident_reports_file_roundtrip() {
    let path = tmp("test_disaster_015.bin");
    let reports = vec![
        IncidentReport {
            incident_id: 1001,
            incident_type: IncidentType::Flood,
            status: ResponseStatus::Monitoring,
            location: Location {
                lat_micro: 29_760_427,
                lon_micro: -95_369_803,
                radius_m: 10_000,
                description: String::from("Houston flood plain sector 4"),
            },
            severity: 5,
            affected_count: 1_200,
            reported_at: 1_700_001_000_u64,
        },
        IncidentReport {
            incident_id: 1002,
            incident_type: IncidentType::Wildfire,
            status: ResponseStatus::Responding,
            location: Location {
                lat_micro: 33_749_249,
                lon_micro: -117_867_833,
                radius_m: 25_000,
                description: String::from("Orange County hillside perimeter"),
            },
            severity: 8,
            affected_count: 3_500,
            reported_at: 1_700_002_000_u64,
        },
        IncidentReport {
            incident_id: 1003,
            incident_type: IncidentType::Earthquake,
            status: ResponseStatus::Contained,
            location: Location {
                lat_micro: 37_338_208,
                lon_micro: -121_886_329,
                radius_m: 15_000,
                description: String::from("San Jose seismic zone"),
            },
            severity: 6,
            affected_count: 900,
            reported_at: 1_700_003_000_u64,
        },
    ];
    encode_to_file(&reports, &path).expect("Failed to encode Vec<IncidentReport> to file");
    let decoded: Vec<IncidentReport> =
        decode_from_file(&path).expect("Failed to decode Vec<IncidentReport> from file");
    assert_eq!(reports, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 16: Overwrite test - encode twice to same path, decode gets second
#[test]
fn test_overwrite_same_path_gets_second_value() {
    let path = tmp("test_disaster_016.bin");
    let first = IncidentReport {
        incident_id: 7001,
        incident_type: IncidentType::Infrastructure,
        status: ResponseStatus::Monitoring,
        location: Location {
            lat_micro: 41_878_113,
            lon_micro: -87_629_799,
            radius_m: 500,
            description: String::from("Chicago bridge sector - first report"),
        },
        severity: 3,
        affected_count: 150,
        reported_at: 1_700_010_000_u64,
    };
    let second = IncidentReport {
        incident_id: 7001,
        incident_type: IncidentType::Infrastructure,
        status: ResponseStatus::Responding,
        location: Location {
            lat_micro: 41_878_113,
            lon_micro: -87_629_799,
            radius_m: 500,
            description: String::from("Chicago bridge sector - updated report"),
        },
        severity: 6,
        affected_count: 400,
        reported_at: 1_700_011_000_u64,
    };
    encode_to_file(&first, &path).expect("Failed to encode first IncidentReport to file");
    encode_to_file(&second, &path).expect("Failed to encode second IncidentReport to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode overwritten IncidentReport from file");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 17: Wildfire response plan
#[test]
fn test_wildfire_response_plan_file() {
    let path = tmp("test_disaster_017.bin");
    let value = ResponsePlan {
        plan_id: 8001,
        incident_id: 2001,
        resources: vec![
            ResourceUnit {
                unit_id: 301,
                unit_type: String::from("Air Tanker"),
                personnel_count: 2,
                equipment_count: 1,
                available: true,
            },
            ResourceUnit {
                unit_id: 302,
                unit_type: String::from("Ground Crew"),
                personnel_count: 20,
                equipment_count: 15,
                available: true,
            },
        ],
        evacuation_zones: vec![Location {
            lat_micro: 34_200_000,
            lon_micro: -118_500_000,
            radius_m: 8_000,
            description: String::from("Wildfire primary evacuation perimeter"),
        }],
        estimated_duration_h: 48,
    };
    encode_to_file(&value, &path).expect("Failed to encode wildfire ResponsePlan to file");
    let decoded: ResponsePlan =
        decode_from_file(&path).expect("Failed to decode wildfire ResponsePlan from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 18: Earthquake with high severity (10)
#[test]
fn test_earthquake_high_severity_file() {
    let path = tmp("test_disaster_018.bin");
    let value = IncidentReport {
        incident_id: 3001,
        incident_type: IncidentType::Earthquake,
        status: ResponseStatus::Responding,
        location: Location {
            lat_micro: 35_689_487,
            lon_micro: 139_691_706,
            radius_m: 50_000,
            description: String::from("Tokyo metropolitan area - major quake"),
        },
        severity: 10,
        affected_count: 500_000,
        reported_at: 1_700_020_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode high-severity earthquake to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode high-severity earthquake from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.severity, 10);
    std::fs::remove_file(&path).ok();
}

// Test 19: Flood with large affected count
#[test]
fn test_flood_large_affected_count_file() {
    let path = tmp("test_disaster_019.bin");
    let value = IncidentReport {
        incident_id: 4001,
        incident_type: IncidentType::Flood,
        status: ResponseStatus::Responding,
        location: Location {
            lat_micro: 23_810_332,
            lon_micro: 90_412_521,
            radius_m: 200_000,
            description: String::from("Dhaka river basin - monsoon flooding"),
        },
        severity: 9,
        affected_count: u32::MAX,
        reported_at: 1_700_030_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode flood with large affected count");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode flood with large affected count");
    assert_eq!(value, decoded);
    assert_eq!(decoded.affected_count, u32::MAX);
    std::fs::remove_file(&path).ok();
}

// Test 20: Chemical spill containment
#[test]
fn test_chemical_spill_containment_file() {
    let path = tmp("test_disaster_020.bin");
    let value = IncidentReport {
        incident_id: 5001,
        incident_type: IncidentType::ChemicalSpill,
        status: ResponseStatus::Contained,
        location: Location {
            lat_micro: 51_507_351,
            lon_micro: -122_127_960,
            radius_m: 300,
            description: String::from("Industrial plant containment zone"),
        },
        severity: 8,
        affected_count: 75,
        reported_at: 1_700_040_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode ChemicalSpill Contained to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode ChemicalSpill Contained from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.status, ResponseStatus::Contained);
    std::fs::remove_file(&path).ok();
}

// Test 21: Infrastructure failure recovery
#[test]
fn test_infrastructure_failure_recovery_file() {
    let path = tmp("test_disaster_021.bin");
    let value = IncidentReport {
        incident_id: 6001,
        incident_type: IncidentType::Infrastructure,
        status: ResponseStatus::Recovery,
        location: Location {
            lat_micro: 48_856_613,
            lon_micro: 2_352_222,
            radius_m: 1_200,
            description: String::from("Paris metro grid failure zone"),
        },
        severity: 5,
        affected_count: 25_000,
        reported_at: 1_700_050_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode Infrastructure Recovery to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode Infrastructure Recovery from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.status, ResponseStatus::Recovery);
    std::fs::remove_file(&path).ok();
}

// Test 22: Mass event monitoring
#[test]
fn test_mass_event_monitoring_file() {
    let path = tmp("test_disaster_022.bin");
    let value = IncidentReport {
        incident_id: 7100,
        incident_type: IncidentType::MassEvent,
        status: ResponseStatus::Monitoring,
        location: Location {
            lat_micro: 51_500_729,
            lon_micro: -121_775_803,
            radius_m: 5_000,
            description: String::from("Stadium district - mass gathering event"),
        },
        severity: 2,
        affected_count: 80_000,
        reported_at: 1_700_060_000_u64,
    };
    encode_to_file(&value, &path).expect("Failed to encode MassEvent Monitoring to file");
    let decoded: IncidentReport =
        decode_from_file(&path).expect("Failed to decode MassEvent Monitoring from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.incident_type, IncidentType::MassEvent);
    assert_eq!(decoded.status, ResponseStatus::Monitoring);
    std::fs::remove_file(&path).ok();
}

// Additional tests to reach 22 total (tests 23-26 renamed to match domain intent):

// Test 23 (encode_to_vec / decode_from_slice companion): Coastal evacuation zone
#[test]
fn test_coastal_evacuation_zone_file() {
    let path = tmp("test_disaster_023.bin");
    let value = Location {
        lat_micro: 25_774_266,
        lon_micro: -80_193_659,
        radius_m: 12_000,
        description: String::from("Miami Beach coastal evacuation zone"),
    };
    encode_to_file(&value, &path).expect("Failed to encode coastal evacuation zone to file");
    let decoded: Location =
        decode_from_file(&path).expect("Failed to decode coastal evacuation zone from file");
    assert_eq!(value, decoded);
    // Cross-check with encode_to_vec / decode_from_slice
    let encoded = encode_to_vec(&value).expect("Failed to encode coastal zone to vec");
    let (from_slice, _): (Location, _) =
        decode_from_slice(&encoded).expect("Failed to decode coastal zone from slice");
    assert_eq!(value, from_slice);
    std::fs::remove_file(&path).ok();
}

// Test 24: Mountain rescue unit
#[test]
fn test_mountain_rescue_unit_file() {
    let path = tmp("test_disaster_024.bin");
    let value = ResourceUnit {
        unit_id: 999,
        unit_type: String::from("Mountain Search and Rescue"),
        personnel_count: 12,
        equipment_count: 30,
        available: true,
    };
    encode_to_file(&value, &path).expect("Failed to encode mountain rescue unit to file");
    let decoded: ResourceUnit =
        decode_from_file(&path).expect("Failed to decode mountain rescue unit from file");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 25: Zero personnel unit (equipment only)
#[test]
fn test_zero_personnel_equipment_only_unit_file() {
    let path = tmp("test_disaster_025.bin");
    let value = ResourceUnit {
        unit_id: 555,
        unit_type: String::from("Autonomous Drone Fleet"),
        personnel_count: 0,
        equipment_count: 50,
        available: true,
    };
    encode_to_file(&value, &path).expect("Failed to encode zero-personnel unit to file");
    let decoded: ResourceUnit =
        decode_from_file(&path).expect("Failed to decode zero-personnel unit from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.personnel_count, 0);
    assert_eq!(decoded.equipment_count, 50);
    std::fs::remove_file(&path).ok();
}

// Test 26: 10-zone evacuation plan
#[test]
fn test_ten_zone_evacuation_plan_file() {
    let path = tmp("test_disaster_026.bin");
    let evacuation_zones: Vec<Location> = (0..10)
        .map(|i| Location {
            lat_micro: 35_000_000 + (i * 10_000),
            lon_micro: 139_000_000 + (i * 10_000),
            radius_m: 1_000 * (i as u32 + 1),
            description: format!("Evacuation zone sector {}", i + 1),
        })
        .collect();
    let value = ResponsePlan {
        plan_id: 9999,
        incident_id: 8888,
        resources: vec![],
        evacuation_zones,
        estimated_duration_h: 72,
    };
    encode_to_file(&value, &path).expect("Failed to encode 10-zone evacuation plan to file");
    let decoded: ResponsePlan =
        decode_from_file(&path).expect("Failed to decode 10-zone evacuation plan from file");
    assert_eq!(value, decoded);
    assert_eq!(decoded.evacuation_zones.len(), 10);
    std::fs::remove_file(&path).ok();
}

// Test 27: Multi-incident disaster scenario
#[test]
fn test_multi_incident_disaster_scenario_file() {
    let path = tmp("test_disaster_027.bin");
    // Compound disaster: hurricane triggers flooding and infrastructure failures
    let incidents = vec![
        IncidentReport {
            incident_id: 10_001,
            incident_type: IncidentType::HurricaneTypoon,
            status: ResponseStatus::Responding,
            location: Location {
                lat_micro: 18_466_633,
                lon_micro: -66_105_736,
                radius_m: 100_000,
                description: String::from("Puerto Rico - hurricane landfall zone"),
            },
            severity: 9,
            affected_count: 1_200_000,
            reported_at: 1_700_100_000_u64,
        },
        IncidentReport {
            incident_id: 10_002,
            incident_type: IncidentType::Flood,
            status: ResponseStatus::Responding,
            location: Location {
                lat_micro: 18_200_000,
                lon_micro: -66_300_000,
                radius_m: 40_000,
                description: String::from("Puerto Rico - inland flooding secondary"),
            },
            severity: 7,
            affected_count: 200_000,
            reported_at: 1_700_101_000_u64,
        },
        IncidentReport {
            incident_id: 10_003,
            incident_type: IncidentType::Infrastructure,
            status: ResponseStatus::Monitoring,
            location: Location {
                lat_micro: 18_400_000,
                lon_micro: -66_000_000,
                radius_m: 5_000,
                description: String::from("Puerto Rico - power grid failure zone"),
            },
            severity: 8,
            affected_count: 800_000,
            reported_at: 1_700_102_000_u64,
        },
    ];
    encode_to_file(&incidents, &path)
        .expect("Failed to encode multi-incident disaster scenario to file");
    let decoded: Vec<IncidentReport> =
        decode_from_file(&path).expect("Failed to decode multi-incident disaster scenario");
    assert_eq!(incidents, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].incident_type, IncidentType::HurricaneTypoon);
    assert_eq!(decoded[1].incident_type, IncidentType::Flood);
    assert_eq!(decoded[2].incident_type, IncidentType::Infrastructure);
    std::fs::remove_file(&path).ok();
}
