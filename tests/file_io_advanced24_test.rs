//! Advanced file I/O tests for wildlife tracking / ecology domain

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
enum Species {
    Wolf,
    Bear,
    Eagle,
    Salmon,
    Elk,
    Lynx,
    Bison,
    Crane,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum HabitatType {
    Forest,
    Grassland,
    Wetland,
    Alpine,
    Coastal,
    Desert,
    Tundra,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GpsLocation {
    lat_micro: i32,
    lon_micro: i32,
    alt_m: i16,
    accuracy_m: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AnimalTag {
    tag_id: u32,
    species: Species,
    weight_g: u32,
    age_months: u16,
    sex_female: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TrackingRecord {
    record_id: u64,
    tag: AnimalTag,
    location: GpsLocation,
    habitat: HabitatType,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MigrationRoute {
    route_id: u64,
    species: Species,
    waypoints: Vec<GpsLocation>,
    distance_km: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PopulationSurvey {
    survey_id: u64,
    region: String,
    species_counts: Vec<(u8, u32)>,
    surveyed_at: u64,
}

// --- Species roundtrip tests (8 tests, one per variant) ---

#[test]
fn test_species_wolf_file_roundtrip() {
    let path = tmp("wildlife_species_wolf.bin");
    let value = Species::Wolf;
    encode_to_file(&value, &path).expect("encode Species::Wolf failed");
    let decoded: Species = decode_from_file(&path).expect("decode Species::Wolf failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_species_bear_file_roundtrip() {
    let path = tmp("wildlife_species_bear.bin");
    let value = Species::Bear;
    encode_to_file(&value, &path).expect("encode Species::Bear failed");
    let decoded: Species = decode_from_file(&path).expect("decode Species::Bear failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_species_eagle_file_roundtrip() {
    let path = tmp("wildlife_species_eagle.bin");
    let value = Species::Eagle;
    encode_to_file(&value, &path).expect("encode Species::Eagle failed");
    let decoded: Species = decode_from_file(&path).expect("decode Species::Eagle failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_species_salmon_file_roundtrip() {
    let path = tmp("wildlife_species_salmon.bin");
    let value = Species::Salmon;
    encode_to_file(&value, &path).expect("encode Species::Salmon failed");
    let decoded: Species = decode_from_file(&path).expect("decode Species::Salmon failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// --- HabitatType roundtrip tests ---

#[test]
fn test_habitat_forest_file_roundtrip() {
    let path = tmp("wildlife_habitat_forest.bin");
    let value = HabitatType::Forest;
    encode_to_file(&value, &path).expect("encode HabitatType::Forest failed");
    let decoded: HabitatType = decode_from_file(&path).expect("decode HabitatType::Forest failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_habitat_tundra_file_roundtrip() {
    let path = tmp("wildlife_habitat_tundra.bin");
    let value = HabitatType::Tundra;
    encode_to_file(&value, &path).expect("encode HabitatType::Tundra failed");
    let decoded: HabitatType = decode_from_file(&path).expect("decode HabitatType::Tundra failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_habitat_wetland_file_roundtrip() {
    let path = tmp("wildlife_habitat_wetland.bin");
    let value = HabitatType::Wetland;
    encode_to_file(&value, &path).expect("encode HabitatType::Wetland failed");
    let decoded: HabitatType = decode_from_file(&path).expect("decode HabitatType::Wetland failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// --- GpsLocation file roundtrip ---

#[test]
fn test_gps_location_file_roundtrip() {
    let path = tmp("wildlife_gps_location.bin");
    let loc = GpsLocation {
        lat_micro: 47_123_456,
        lon_micro: -122_456_789,
        alt_m: 850,
        accuracy_m: 5,
    };
    encode_to_file(&loc, &path).expect("encode GpsLocation failed");
    let decoded: GpsLocation = decode_from_file(&path).expect("decode GpsLocation failed");
    assert_eq!(loc, decoded);
    std::fs::remove_file(&path).ok();
}

// --- AnimalTag file roundtrip ---

#[test]
fn test_animal_tag_file_roundtrip() {
    let path = tmp("wildlife_animal_tag.bin");
    let tag = AnimalTag {
        tag_id: 100_001,
        species: Species::Elk,
        weight_g: 320_000,
        age_months: 48,
        sex_female: false,
    };
    encode_to_file(&tag, &path).expect("encode AnimalTag failed");
    let decoded: AnimalTag = decode_from_file(&path).expect("decode AnimalTag failed");
    assert_eq!(tag, decoded);
    std::fs::remove_file(&path).ok();
}

// --- TrackingRecord file roundtrip ---

#[test]
fn test_tracking_record_file_roundtrip() {
    let path = tmp("wildlife_tracking_record.bin");
    let record = TrackingRecord {
        record_id: 9_999_000_001,
        tag: AnimalTag {
            tag_id: 200_042,
            species: Species::Lynx,
            weight_g: 11_500,
            age_months: 36,
            sex_female: true,
        },
        location: GpsLocation {
            lat_micro: 60_001_000,
            lon_micro: 25_500_000,
            alt_m: 120,
            accuracy_m: 3,
        },
        habitat: HabitatType::Forest,
        timestamp_s: 1_700_000_000,
    };
    encode_to_file(&record, &path).expect("encode TrackingRecord failed");
    let decoded: TrackingRecord = decode_from_file(&path).expect("decode TrackingRecord failed");
    assert_eq!(record, decoded);
    std::fs::remove_file(&path).ok();
}

// --- MigrationRoute with empty waypoints ---

#[test]
fn test_migration_route_empty_waypoints() {
    let path = tmp("wildlife_migration_empty.bin");
    let route = MigrationRoute {
        route_id: 1,
        species: Species::Crane,
        waypoints: vec![],
        distance_km: 0,
    };
    encode_to_file(&route, &path).expect("encode empty MigrationRoute failed");
    let decoded: MigrationRoute =
        decode_from_file(&path).expect("decode empty MigrationRoute failed");
    assert_eq!(route, decoded);
    assert!(decoded.waypoints.is_empty());
    std::fs::remove_file(&path).ok();
}

// --- MigrationRoute with 10 waypoints ---

#[test]
fn test_migration_route_ten_waypoints() {
    let path = tmp("wildlife_migration_ten.bin");
    let waypoints: Vec<GpsLocation> = (0..10)
        .map(|i| GpsLocation {
            lat_micro: 40_000_000 + i * 500_000,
            lon_micro: 20_000_000 + i * 300_000,
            alt_m: 200 + i as i16 * 50,
            accuracy_m: 10,
        })
        .collect();
    let route = MigrationRoute {
        route_id: 42,
        species: Species::Crane,
        waypoints,
        distance_km: 1_500,
    };
    encode_to_file(&route, &path).expect("encode 10-waypoint MigrationRoute failed");
    let decoded: MigrationRoute =
        decode_from_file(&path).expect("decode 10-waypoint MigrationRoute failed");
    assert_eq!(route, decoded);
    assert_eq!(decoded.waypoints.len(), 10);
    std::fs::remove_file(&path).ok();
}

// --- PopulationSurvey with all species ---

#[test]
fn test_population_survey_all_species() {
    let path = tmp("wildlife_survey_all_species.bin");
    // species_counts: (species variant index as u8, count)
    let species_counts: Vec<(u8, u32)> = vec![
        (0, 45),   // Wolf
        (1, 12),   // Bear
        (2, 200),  // Eagle
        (3, 3500), // Salmon
        (4, 180),  // Elk
        (5, 8),    // Lynx
        (6, 60),   // Bison
        (7, 75),   // Crane
    ];
    let survey = PopulationSurvey {
        survey_id: 2024_0001,
        region: "Yellowstone National Park".to_string(),
        species_counts,
        surveyed_at: 1_710_000_000,
    };
    encode_to_file(&survey, &path).expect("encode PopulationSurvey all species failed");
    let decoded: PopulationSurvey =
        decode_from_file(&path).expect("decode PopulationSurvey all species failed");
    assert_eq!(survey, decoded);
    assert_eq!(decoded.species_counts.len(), 8);
    std::fs::remove_file(&path).ok();
}

// --- Vec<TrackingRecord> file roundtrip ---

#[test]
fn test_vec_tracking_records_file_roundtrip() {
    let path = tmp("wildlife_vec_tracking_records.bin");
    let records: Vec<TrackingRecord> = (0..5)
        .map(|i| TrackingRecord {
            record_id: 1000 + i,
            tag: AnimalTag {
                tag_id: 300_000 + i as u32,
                species: Species::Bear,
                weight_g: 200_000 + i as u32 * 5_000,
                age_months: 24 + i as u16,
                sex_female: i % 2 == 0,
            },
            location: GpsLocation {
                lat_micro: 55_000_000 + i as i32 * 10_000,
                lon_micro: 37_000_000 + i as i32 * 10_000,
                alt_m: 300 + i as i16 * 10,
                accuracy_m: 8,
            },
            habitat: HabitatType::Forest,
            timestamp_s: 1_700_100_000 + i * 3600,
        })
        .collect();
    encode_to_file(&records, &path).expect("encode Vec<TrackingRecord> failed");
    let decoded: Vec<TrackingRecord> =
        decode_from_file(&path).expect("decode Vec<TrackingRecord> failed");
    assert_eq!(records, decoded);
    assert_eq!(decoded.len(), 5);
    std::fs::remove_file(&path).ok();
}

// --- Overwrite test ---

#[test]
fn test_overwrite_tracking_record() {
    let path = tmp("wildlife_overwrite_test.bin");
    let first = TrackingRecord {
        record_id: 1,
        tag: AnimalTag {
            tag_id: 1,
            species: Species::Wolf,
            weight_g: 35_000,
            age_months: 24,
            sex_female: false,
        },
        location: GpsLocation {
            lat_micro: 45_000_000,
            lon_micro: -110_000_000,
            alt_m: 2_100,
            accuracy_m: 6,
        },
        habitat: HabitatType::Alpine,
        timestamp_s: 1_700_000_000,
    };
    let second = TrackingRecord {
        record_id: 2,
        tag: AnimalTag {
            tag_id: 2,
            species: Species::Bison,
            weight_g: 900_000,
            age_months: 60,
            sex_female: true,
        },
        location: GpsLocation {
            lat_micro: 46_000_000,
            lon_micro: -109_000_000,
            alt_m: 1_800,
            accuracy_m: 4,
        },
        habitat: HabitatType::Grassland,
        timestamp_s: 1_700_100_000,
    };
    encode_to_file(&first, &path).expect("first encode failed");
    encode_to_file(&second, &path).expect("overwrite encode failed");
    let decoded: TrackingRecord = decode_from_file(&path).expect("decode after overwrite failed");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// --- Wolf pack tracking (3 wolves) ---

#[test]
fn test_wolf_pack_three_wolves() {
    let path = tmp("wildlife_wolf_pack.bin");
    let pack: Vec<AnimalTag> = vec![
        AnimalTag {
            tag_id: 400_001,
            species: Species::Wolf,
            weight_g: 42_000,
            age_months: 36,
            sex_female: false,
        },
        AnimalTag {
            tag_id: 400_002,
            species: Species::Wolf,
            weight_g: 38_000,
            age_months: 30,
            sex_female: true,
        },
        AnimalTag {
            tag_id: 400_003,
            species: Species::Wolf,
            weight_g: 28_000,
            age_months: 14,
            sex_female: false,
        },
    ];
    encode_to_file(&pack, &path).expect("encode wolf pack failed");
    let decoded: Vec<AnimalTag> = decode_from_file(&path).expect("decode wolf pack failed");
    assert_eq!(pack, decoded);
    assert_eq!(decoded.len(), 3);
    assert!(decoded.iter().all(|t| matches!(t.species, Species::Wolf)));
    std::fs::remove_file(&path).ok();
}

// --- Eagle migration with 20 waypoints ---

#[test]
fn test_eagle_migration_twenty_waypoints() {
    let path = tmp("wildlife_eagle_migration20.bin");
    let waypoints: Vec<GpsLocation> = (0..20)
        .map(|i| GpsLocation {
            lat_micro: 65_000_000 - i * 1_000_000,
            lon_micro: 15_000_000 + i * 200_000,
            alt_m: 800 + i as i16 * 30,
            accuracy_m: 12,
        })
        .collect();
    let route = MigrationRoute {
        route_id: 200,
        species: Species::Eagle,
        waypoints,
        distance_km: 4_200,
    };
    encode_to_file(&route, &path).expect("encode eagle migration failed");
    let decoded: MigrationRoute = decode_from_file(&path).expect("decode eagle migration failed");
    assert_eq!(route, decoded);
    assert_eq!(decoded.waypoints.len(), 20);
    std::fs::remove_file(&path).ok();
}

// --- Salmon spawning route ---

#[test]
fn test_salmon_spawning_route() {
    let path = tmp("wildlife_salmon_spawning.bin");
    let waypoints: Vec<GpsLocation> = vec![
        GpsLocation {
            lat_micro: 58_500_000,
            lon_micro: -134_000_000,
            alt_m: 0,
            accuracy_m: 20,
        },
        GpsLocation {
            lat_micro: 58_600_000,
            lon_micro: -133_800_000,
            alt_m: 5,
            accuracy_m: 20,
        },
        GpsLocation {
            lat_micro: 58_700_000,
            lon_micro: -133_600_000,
            alt_m: 12,
            accuracy_m: 15,
        },
    ];
    let route = MigrationRoute {
        route_id: 300,
        species: Species::Salmon,
        waypoints,
        distance_km: 240,
    };
    encode_to_file(&route, &path).expect("encode salmon spawning route failed");
    let decoded: MigrationRoute =
        decode_from_file(&path).expect("decode salmon spawning route failed");
    assert_eq!(route, decoded);
    assert!(matches!(decoded.species, Species::Salmon));
    std::fs::remove_file(&path).ok();
}

// --- Bear weight extreme (500 kg = 500_000 g) ---

#[test]
fn test_bear_extreme_weight() {
    let path = tmp("wildlife_bear_extreme_weight.bin");
    let tag = AnimalTag {
        tag_id: 500_001,
        species: Species::Bear,
        weight_g: 500_000,
        age_months: 120,
        sex_female: false,
    };
    encode_to_file(&tag, &path).expect("encode extreme bear weight failed");
    let decoded: AnimalTag = decode_from_file(&path).expect("decode extreme bear weight failed");
    assert_eq!(tag, decoded);
    assert_eq!(decoded.weight_g, 500_000);
    std::fs::remove_file(&path).ok();
}

// --- Juvenile animal (age 0) ---

#[test]
fn test_juvenile_animal_age_zero() {
    let path = tmp("wildlife_juvenile_age_zero.bin");
    let tag = AnimalTag {
        tag_id: 600_001,
        species: Species::Elk,
        weight_g: 18_000,
        age_months: 0,
        sex_female: true,
    };
    encode_to_file(&tag, &path).expect("encode juvenile animal failed");
    let decoded: AnimalTag = decode_from_file(&path).expect("decode juvenile animal failed");
    assert_eq!(tag, decoded);
    assert_eq!(decoded.age_months, 0);
    std::fs::remove_file(&path).ok();
}

// --- Female vs male tag produce distinct bytes ---

#[test]
fn test_female_vs_male_tag_distinct_bytes() {
    let female = AnimalTag {
        tag_id: 700_001,
        species: Species::Wolf,
        weight_g: 32_000,
        age_months: 24,
        sex_female: true,
    };
    let male = AnimalTag {
        tag_id: 700_001,
        species: Species::Wolf,
        weight_g: 32_000,
        age_months: 24,
        sex_female: false,
    };
    let female_bytes = encode_to_vec(&female).expect("encode female tag failed");
    let male_bytes = encode_to_vec(&male).expect("encode male tag failed");
    assert_ne!(female_bytes, male_bytes);

    let (decoded_female, _): (AnimalTag, _) =
        decode_from_slice(&female_bytes).expect("decode female tag failed");
    let (decoded_male, _): (AnimalTag, _) =
        decode_from_slice(&male_bytes).expect("decode male tag failed");
    assert_eq!(female, decoded_female);
    assert_eq!(male, decoded_male);
}

// --- Tundra vs forest habitat produce distinct bytes ---

#[test]
fn test_tundra_vs_forest_habitat_distinct_bytes() {
    let tundra_bytes = encode_to_vec(&HabitatType::Tundra).expect("encode Tundra failed");
    let forest_bytes = encode_to_vec(&HabitatType::Forest).expect("encode Forest failed");
    assert_ne!(tundra_bytes, forest_bytes);

    let (decoded_tundra, _): (HabitatType, _) =
        decode_from_slice(&tundra_bytes).expect("decode Tundra failed");
    let (decoded_forest, _): (HabitatType, _) =
        decode_from_slice(&forest_bytes).expect("decode Forest failed");
    assert!(matches!(decoded_tundra, HabitatType::Tundra));
    assert!(matches!(decoded_forest, HabitatType::Forest));
}

// --- Long migration route (1000 km) ---

#[test]
fn test_long_migration_route_1000km() {
    let path = tmp("wildlife_long_migration_1000km.bin");
    let waypoints: Vec<GpsLocation> = (0..50)
        .map(|i| GpsLocation {
            lat_micro: 30_000_000 + i * 400_000,
            lon_micro: -100_000_000 + i * 300_000,
            alt_m: 500 + i as i16 * 20,
            accuracy_m: 15,
        })
        .collect();
    let route = MigrationRoute {
        route_id: 1000,
        species: Species::Bison,
        waypoints,
        distance_km: 1_000,
    };
    encode_to_file(&route, &path).expect("encode long migration route failed");
    let decoded: MigrationRoute =
        decode_from_file(&path).expect("decode long migration route failed");
    assert_eq!(route, decoded);
    assert_eq!(decoded.distance_km, 1_000);
    assert_eq!(decoded.waypoints.len(), 50);
    std::fs::remove_file(&path).ok();
}

// --- Rare species survey ---

#[test]
fn test_rare_species_survey() {
    let path = tmp("wildlife_rare_species_survey.bin");
    let survey = PopulationSurvey {
        survey_id: 2024_9999,
        region: "Remote Boreal Wilderness".to_string(),
        species_counts: vec![
            (5, 2), // Lynx — very rare
            (7, 1), // Crane — critically endangered in region
        ],
        surveyed_at: 1_715_000_000,
    };
    encode_to_file(&survey, &path).expect("encode rare species survey failed");
    let decoded: PopulationSurvey =
        decode_from_file(&path).expect("decode rare species survey failed");
    assert_eq!(survey, decoded);
    assert_eq!(decoded.species_counts.len(), 2);
    assert!(decoded.species_counts.iter().all(|(_, count)| *count <= 2));
    std::fs::remove_file(&path).ok();
}

// --- File existence check after write ---

#[test]
fn test_file_exists_after_encode() {
    let path = tmp("wildlife_file_existence_check.bin");
    let loc = GpsLocation {
        lat_micro: 51_500_000,
        lon_micro: -0_127_000,
        alt_m: 11,
        accuracy_m: 2,
    };
    assert!(!path.exists() || std::fs::remove_file(&path).is_ok());
    encode_to_file(&loc, &path).expect("encode for existence check failed");
    assert!(path.exists(), "file should exist after encode_to_file");
    let decoded: GpsLocation = decode_from_file(&path).expect("decode for existence check failed");
    assert_eq!(loc, decoded);
    std::fs::remove_file(&path).ok();
}

// --- Large survey (all 8 species, high counts) ---

#[test]
fn test_large_survey_all_eight_species() {
    let path = tmp("wildlife_large_survey_all8.bin");
    let species_counts: Vec<(u8, u32)> = vec![
        (0, 120),    // Wolf
        (1, 55),     // Bear
        (2, 890),    // Eagle
        (3, 50_000), // Salmon
        (4, 750),    // Elk
        (5, 30),     // Lynx
        (6, 400),    // Bison
        (7, 310),    // Crane
    ];
    let survey = PopulationSurvey {
        survey_id: 2025_0001,
        region: "Greater Yellowstone Ecosystem".to_string(),
        species_counts,
        surveyed_at: 1_720_000_000,
    };
    encode_to_file(&survey, &path).expect("encode large survey failed");
    let decoded: PopulationSurvey = decode_from_file(&path).expect("decode large survey failed");
    assert_eq!(survey, decoded);
    assert_eq!(decoded.species_counts.len(), 8);
    let total: u32 = decoded.species_counts.iter().map(|(_, c)| c).sum();
    assert!(total > 50_000);
    std::fs::remove_file(&path).ok();
}
