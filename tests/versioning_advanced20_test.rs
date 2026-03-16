//! Aquaculture / fisheries management versioning tests for OxiCode (set 20).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! and Version across fish species, water quality readings, farm stock records,
//! and harvest management — verifying roundtrips, version preservation, schema
//! evolution (V1 → V2), ordering, Vec serialisation, seasonal versioning,
//! consumed-bytes accounting, and various domain-specific edge cases.

#![cfg(feature = "versioning")]
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
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum FishSpecies {
    Salmon,
    Trout,
    Tilapia,
    Catfish,
    Shrimp,
    Oyster,
    Crab,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WaterQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

/// Water reading with fixed-point values:
/// - `temperature_mc`: temperature in milli-Celsius (e.g. 25_000 = 25.000 °C)
/// - `ph_micro`: pH × 1_000_000 (e.g. 7_200_000 = 7.2)
/// - `oxygen_ppb`: dissolved oxygen in ppb
/// - `salinity_ppm`: salinity in ppm
/// - `timestamp_s`: Unix epoch seconds
#[derive(Debug, PartialEq, Encode, Decode)]
struct WaterReading {
    pond_id: u32,
    temperature_mc: i32,
    ph_micro: u32,
    oxygen_ppb: u32,
    salinity_ppm: u32,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FarmStockV1 {
    farm_id: u64,
    species: FishSpecies,
    count: u32,
    avg_weight_g: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FarmStockV2 {
    farm_id: u64,
    species: FishSpecies,
    count: u32,
    avg_weight_g: u32,
    water_quality: WaterQuality,
    feed_kg_per_day: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HarvestRecord {
    harvest_id: u64,
    farm_id: u64,
    species: FishSpecies,
    quantity_kg: u32,
    price_cents_per_kg: u32,
    harvested_at: u64,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// Test 1 — FarmStockV1 round-trips under version 1.0.0
#[test]
fn test_farm_stock_v1_version_1_0_0_roundtrip() {
    let version = Version::new(1, 0, 0);
    let stock = FarmStockV1 {
        farm_id: 100,
        species: FishSpecies::Salmon,
        count: 5_000,
        avg_weight_g: 2_500,
    };
    let encoded =
        encode_versioned_value(&stock, version).expect("encode_versioned_value FarmStockV1 failed");
    let (decoded, ver, _consumed): (FarmStockV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value FarmStockV1 failed");
    assert_eq!(decoded, stock);
    assert_eq!(ver, version);
}

/// Test 2 — FarmStockV2 round-trips under version 2.0.0
#[test]
fn test_farm_stock_v2_version_2_0_0_roundtrip() {
    let version = Version::new(2, 0, 0);
    let stock = FarmStockV2 {
        farm_id: 200,
        species: FishSpecies::Tilapia,
        count: 12_000,
        avg_weight_g: 800,
        water_quality: WaterQuality::Good,
        feed_kg_per_day: 150,
    };
    let encoded =
        encode_versioned_value(&stock, version).expect("encode_versioned_value FarmStockV2 failed");
    let (decoded, ver, _consumed): (FarmStockV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value FarmStockV2 failed");
    assert_eq!(decoded, stock);
    assert_eq!(ver, version);
}

/// Test 3 — each FishSpecies variant survives a versioned roundtrip
#[test]
fn test_each_fish_species_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let all_species = [
        FishSpecies::Salmon,
        FishSpecies::Trout,
        FishSpecies::Tilapia,
        FishSpecies::Catfish,
        FishSpecies::Shrimp,
        FishSpecies::Oyster,
        FishSpecies::Crab,
    ];
    for species in all_species {
        let stock = FarmStockV1 {
            farm_id: 1,
            species,
            count: 100,
            avg_weight_g: 500,
        };
        let encoded = encode_versioned_value(&stock, version)
            .expect("encode_versioned_value for species failed");
        let (decoded, ver, _): (FarmStockV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value for species failed");
        assert_eq!(decoded, stock);
        assert_eq!(ver, version);
    }
}

/// Test 4 — each WaterQuality variant survives a versioned roundtrip
#[test]
fn test_each_water_quality_versioned_roundtrip() {
    let version = Version::new(2, 0, 0);
    let qualities = [
        WaterQuality::Excellent,
        WaterQuality::Good,
        WaterQuality::Fair,
        WaterQuality::Poor,
        WaterQuality::Critical,
    ];
    for quality in qualities {
        let stock = FarmStockV2 {
            farm_id: 42,
            species: FishSpecies::Trout,
            count: 3_000,
            avg_weight_g: 1_200,
            water_quality: quality,
            feed_kg_per_day: 80,
        };
        let encoded = encode_versioned_value(&stock, version)
            .expect("encode_versioned_value for water quality failed");
        let (decoded, ver, _): (FarmStockV2, Version, usize) = decode_versioned_value(&encoded)
            .expect("decode_versioned_value for water quality failed");
        assert_eq!(decoded, stock);
        assert_eq!(ver, version);
    }
}

/// Test 5 — WaterReading survives a versioned roundtrip
#[test]
fn test_water_reading_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let reading = WaterReading {
        pond_id: 7,
        temperature_mc: 22_500, // 22.5 °C
        ph_micro: 7_400_000,    // pH 7.4
        oxygen_ppb: 8_200_000,  // 8.2 mg/L expressed as ppb
        salinity_ppm: 3_500,    // 3.5 ppt
        timestamp_s: 1_700_000_000,
    };
    let encoded = encode_versioned_value(&reading, version)
        .expect("encode_versioned_value WaterReading failed");
    let (decoded, ver, _): (WaterReading, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value WaterReading failed");
    assert_eq!(decoded, reading);
    assert_eq!(ver, version);
}

/// Test 6 — HarvestRecord survives a versioned roundtrip
#[test]
fn test_harvest_record_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let record = HarvestRecord {
        harvest_id: 9_001,
        farm_id: 300,
        species: FishSpecies::Catfish,
        quantity_kg: 4_500,
        price_cents_per_kg: 850, // $8.50/kg
        harvested_at: 1_710_000_000,
    };
    let encoded = encode_versioned_value(&record, version)
        .expect("encode_versioned_value HarvestRecord failed");
    let (decoded, ver, _): (HarvestRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value HarvestRecord failed");
    assert_eq!(decoded, record);
    assert_eq!(ver, version);
}

/// Test 7 — version triple (major, minor, patch) is fully preserved
#[test]
fn test_version_triple_fully_preserved() {
    let version = Version::new(3, 7, 11);
    let reading = WaterReading {
        pond_id: 1,
        temperature_mc: 18_000,
        ph_micro: 7_000_000,
        oxygen_ppb: 9_000_000,
        salinity_ppm: 0,
        timestamp_s: 1_000,
    };
    let encoded =
        encode_versioned_value(&reading, version).expect("encode for version triple test");
    let (_, ver, _): (WaterReading, Version, usize) =
        decode_versioned_value(&encoded).expect("decode for version triple test");
    assert_eq!(ver.major, 3u16);
    assert_eq!(ver.minor, 7u16);
    assert_eq!(ver.patch, 11u16);
    assert_eq!(ver, version);
}

/// Test 8 — version comparison: v1.0.0 < v2.0.0
#[test]
fn test_version_comparison_v1_less_than_v2() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2, "v1.0.0 must be less than v2.0.0");
    assert!(v2 > v1, "v2.0.0 must be greater than v1.0.0");
    assert!(
        !v1.is_compatible_with(&v2),
        "v1.0.0 must not be compatible with v2.0.0"
    );
    assert!(
        v2.is_breaking_change_from(&v1),
        "v2.0.0 must represent a breaking change from v1.0.0"
    );
}

/// Test 9 — Vec<FarmStockV1> survives a versioned roundtrip
#[test]
fn test_vec_farm_stock_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let stocks = vec![
        FarmStockV1 {
            farm_id: 10,
            species: FishSpecies::Salmon,
            count: 1_000,
            avg_weight_g: 3_000,
        },
        FarmStockV1 {
            farm_id: 11,
            species: FishSpecies::Trout,
            count: 500,
            avg_weight_g: 1_800,
        },
        FarmStockV1 {
            farm_id: 12,
            species: FishSpecies::Shrimp,
            count: 50_000,
            avg_weight_g: 20,
        },
    ];
    let encoded = encode_versioned_value(&stocks, version).expect("encode Vec<FarmStockV1>");
    let (decoded, ver, _): (Vec<FarmStockV1>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode Vec<FarmStockV1>");
    assert_eq!(decoded, stocks);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
}

/// Test 10 — seasonal harvest versioning: v1.0.0 → v1.1.0 for each season
#[test]
fn test_seasonal_harvest_versioning_v1_0_0_to_v1_1_0() {
    let seasons = [
        (Version::new(1, 0, 0), "spring", 1_706_000_000u64),
        (Version::new(1, 1, 0), "summer", 1_714_000_000u64),
        (Version::new(1, 1, 0), "autumn", 1_722_000_000u64),
        (Version::new(1, 1, 0), "winter", 1_730_000_000u64),
    ];
    for (version, _season_name, timestamp) in seasons {
        let record = HarvestRecord {
            harvest_id: timestamp,
            farm_id: 50,
            species: FishSpecies::Salmon,
            quantity_kg: 2_000,
            price_cents_per_kg: 1_200,
            harvested_at: timestamp,
        };
        let encoded = encode_versioned_value(&record, version).expect("encode seasonal harvest");
        let (decoded, ver, _): (HarvestRecord, Version, usize) =
            decode_versioned_value(&encoded).expect("decode seasonal harvest");
        assert_eq!(decoded, record);
        assert_eq!(ver, version);
    }
}

/// Test 11 — critical water quality alert encoded and verified
#[test]
fn test_critical_water_quality_alert() {
    let version = Version::new(2, 0, 0);
    let alert = FarmStockV2 {
        farm_id: 999,
        species: FishSpecies::Salmon,
        count: 10_000,
        avg_weight_g: 2_000,
        water_quality: WaterQuality::Critical,
        feed_kg_per_day: 0, // feeding suspended during critical event
    };
    let encoded =
        encode_versioned_value(&alert, version).expect("encode critical water quality alert");
    let (decoded, ver, _): (FarmStockV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode critical water quality alert");
    assert_eq!(decoded, alert);
    assert_eq!(ver, version);
    assert_eq!(decoded.water_quality, WaterQuality::Critical);
    assert_eq!(decoded.feed_kg_per_day, 0);
}

/// Test 12 — salmon and tilapia produce distinct byte sequences
#[test]
fn test_salmon_vs_tilapia_produce_distinct_bytes() {
    let version = Version::new(1, 0, 0);
    let salmon_stock = FarmStockV1 {
        farm_id: 1,
        species: FishSpecies::Salmon,
        count: 1_000,
        avg_weight_g: 3_000,
    };
    let tilapia_stock = FarmStockV1 {
        farm_id: 1,
        species: FishSpecies::Tilapia,
        count: 1_000,
        avg_weight_g: 3_000,
    };
    let salmon_bytes = encode_versioned_value(&salmon_stock, version).expect("encode salmon");
    let tilapia_bytes = encode_versioned_value(&tilapia_stock, version).expect("encode tilapia");
    assert_ne!(
        salmon_bytes, tilapia_bytes,
        "salmon and tilapia must produce distinct byte sequences"
    );
}

/// Test 13 — high fish count (1 million fish) round-trips correctly
#[test]
fn test_high_count_one_million_fish() {
    let version = Version::new(1, 0, 0);
    let stock = FarmStockV1 {
        farm_id: 77,
        species: FishSpecies::Shrimp,
        count: 1_000_000,
        avg_weight_g: 15,
    };
    let encoded = encode_versioned_value(&stock, version).expect("encode 1M fish");
    let (decoded, ver, _): (FarmStockV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode 1M fish");
    assert_eq!(decoded.count, 1_000_000);
    assert_eq!(ver, version);
}

/// Test 14 — zero avg_weight_g edge case (newly stocked fry)
#[test]
fn test_zero_weight_edge_case_newly_stocked_fry() {
    let version = Version::new(1, 0, 0);
    let stock = FarmStockV1 {
        farm_id: 5,
        species: FishSpecies::Trout,
        count: 200_000,
        avg_weight_g: 0, // fry just added — weight essentially zero
    };
    let encoded = encode_versioned_value(&stock, version).expect("encode zero weight fry");
    let (decoded, ver, _): (FarmStockV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode zero weight fry");
    assert_eq!(decoded.avg_weight_g, 0);
    assert_eq!(decoded.count, 200_000);
    assert_eq!(ver, version);
}

/// Test 15 — maximum temperature reading (tropical marine pond)
#[test]
fn test_max_temperature_reading() {
    let version = Version::new(1, 0, 0);
    // 40.0 °C expressed as milli-Celsius
    let reading = WaterReading {
        pond_id: 3,
        temperature_mc: 40_000,
        ph_micro: 8_100_000,
        oxygen_ppb: 6_500_000,
        salinity_ppm: 35_000,
        timestamp_s: 1_750_000_000,
    };
    let encoded = encode_versioned_value(&reading, version).expect("encode max temperature");
    let (decoded, ver, _): (WaterReading, Version, usize) =
        decode_versioned_value(&encoded).expect("decode max temperature");
    assert_eq!(decoded.temperature_mc, 40_000);
    assert_eq!(ver, version);
}

/// Test 16 — salinity range verification: fresh-water (0 ppm) and brackish (15_000 ppm)
#[test]
fn test_salinity_range_fresh_and_brackish() {
    let version = Version::new(1, 0, 0);
    let freshwater = WaterReading {
        pond_id: 10,
        temperature_mc: 20_000,
        ph_micro: 7_000_000,
        oxygen_ppb: 10_000_000,
        salinity_ppm: 0, // fresh water
        timestamp_s: 1_000,
    };
    let brackish = WaterReading {
        pond_id: 11,
        temperature_mc: 25_000,
        ph_micro: 7_800_000,
        oxygen_ppb: 7_500_000,
        salinity_ppm: 15_000, // brackish water
        timestamp_s: 2_000,
    };
    for reading in [&freshwater, &brackish] {
        let encoded = encode_versioned_value(reading, version).expect("encode salinity reading");
        let (decoded, ver, _): (WaterReading, Version, usize) =
            decode_versioned_value(&encoded).expect("decode salinity reading");
        assert_eq!(&decoded, reading);
        assert_eq!(ver, version);
    }
}

/// Test 17 — feed optimisation: V2 includes feed_kg_per_day absent in V1
#[test]
fn test_feed_optimization_v2_vs_v1() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    let stock_v1 = FarmStockV1 {
        farm_id: 20,
        species: FishSpecies::Catfish,
        count: 8_000,
        avg_weight_g: 600,
    };
    let stock_v2 = FarmStockV2 {
        farm_id: 20,
        species: FishSpecies::Catfish,
        count: 8_000,
        avg_weight_g: 600,
        water_quality: WaterQuality::Good,
        feed_kg_per_day: 240, // optimised feed rate
    };

    let enc_v1 = encode_versioned_value(&stock_v1, v1).expect("encode V1 feed");
    let enc_v2 = encode_versioned_value(&stock_v2, v2).expect("encode V2 feed");

    let (decoded_v1, ver1, _): (FarmStockV1, Version, usize) =
        decode_versioned_value(&enc_v1).expect("decode V1 feed");
    let (decoded_v2, ver2, _): (FarmStockV2, Version, usize) =
        decode_versioned_value(&enc_v2).expect("decode V2 feed");

    assert_eq!(decoded_v1, stock_v1);
    assert_eq!(decoded_v2, stock_v2);
    assert_eq!(ver1, v1);
    assert_eq!(ver2, v2);
    // V2 carries additional management fields not present in V1
    assert_eq!(decoded_v2.feed_kg_per_day, 240);
    assert!(v2 > v1);
}

/// Test 18 — oxygen level boundary: hypoxic threshold (below 3 mg/L)
#[test]
fn test_oxygen_level_hypoxic_boundary() {
    let version = Version::new(1, 0, 0);
    // 2.8 mg/L expressed as ppb (1 mg/L ≈ 1_000_000 ppb for dissolved O₂ by convention)
    let reading = WaterReading {
        pond_id: 8,
        temperature_mc: 28_000,
        ph_micro: 6_500_000,
        oxygen_ppb: 2_800_000, // dangerously low
        salinity_ppm: 1_000,
        timestamp_s: 1_720_000_000,
    };
    let encoded = encode_versioned_value(&reading, version).expect("encode hypoxic reading");
    let (decoded, ver, _): (WaterReading, Version, usize) =
        decode_versioned_value(&encoded).expect("decode hypoxic reading");
    assert_eq!(decoded.oxygen_ppb, 2_800_000);
    assert_eq!(ver, version);
    // Verify it encodes distinct bytes from a healthy reading
    let healthy = WaterReading {
        oxygen_ppb: 9_000_000,
        ..decoded
    };
    let healthy_encoded =
        encode_versioned_value(&healthy, version).expect("encode healthy reading");
    assert_ne!(encoded, healthy_encoded);
}

/// Test 19 — farm upgrade scenario: same farm re-encoded from V1 to V2
#[test]
fn test_farm_upgrade_v1_to_v2_scenario() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    // Original record stored under V1
    let old_record = FarmStockV1 {
        farm_id: 55,
        species: FishSpecies::Oyster,
        count: 100_000,
        avg_weight_g: 80,
    };
    let enc_v1 = encode_versioned_value(&old_record, v1).expect("encode V1 farm");
    let (recovered_v1, ver1, _): (FarmStockV1, Version, usize) =
        decode_versioned_value(&enc_v1).expect("decode V1 farm");
    assert_eq!(recovered_v1, old_record);
    assert_eq!(ver1, v1);

    // After farm upgrade, re-encode with enriched V2 record
    let upgraded = FarmStockV2 {
        farm_id: 55,
        species: FishSpecies::Oyster,
        count: 100_000,
        avg_weight_g: 80,
        water_quality: WaterQuality::Excellent,
        feed_kg_per_day: 50,
    };
    let enc_v2 = encode_versioned_value(&upgraded, v2).expect("encode V2 farm");
    let (recovered_v2, ver2, _): (FarmStockV2, Version, usize) =
        decode_versioned_value(&enc_v2).expect("decode V2 farm");
    assert_eq!(recovered_v2, upgraded);
    assert_eq!(ver2, v2);
    // Versions are ordered correctly
    assert!(ver2 > ver1);
}

/// Test 20 — multi-species farm: Vec<FarmStockV2> with three different species
#[test]
fn test_multi_species_farm_v2_versioned() {
    let version = Version::new(2, 0, 0);
    let farm_records = vec![
        FarmStockV2 {
            farm_id: 30,
            species: FishSpecies::Salmon,
            count: 5_000,
            avg_weight_g: 4_000,
            water_quality: WaterQuality::Excellent,
            feed_kg_per_day: 200,
        },
        FarmStockV2 {
            farm_id: 30,
            species: FishSpecies::Crab,
            count: 20_000,
            avg_weight_g: 250,
            water_quality: WaterQuality::Good,
            feed_kg_per_day: 100,
        },
        FarmStockV2 {
            farm_id: 30,
            species: FishSpecies::Oyster,
            count: 500_000,
            avg_weight_g: 60,
            water_quality: WaterQuality::Good,
            feed_kg_per_day: 30,
        },
    ];
    let encoded =
        encode_versioned_value(&farm_records, version).expect("encode multi-species farm");
    let (decoded, ver, _): (Vec<FarmStockV2>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode multi-species farm");
    assert_eq!(decoded, farm_records);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].species, FishSpecies::Salmon);
    assert_eq!(decoded[1].species, FishSpecies::Crab);
    assert_eq!(decoded[2].species, FishSpecies::Oyster);
}

/// Test 21 — consumed bytes: versioned header overhead is accounted for
#[test]
fn test_consumed_bytes_versioned_header_overhead() {
    let version = Version::new(1, 0, 0);
    let stock = FarmStockV1 {
        farm_id: 1_000,
        species: FishSpecies::Salmon,
        count: 9_999,
        avg_weight_g: 3_500,
    };
    // Encode without version header to measure raw payload size
    let raw_payload = encode_to_vec(&stock).expect("encode_to_vec FarmStockV1");

    // Encode with version header
    let versioned_encoded =
        encode_versioned_value(&stock, version).expect("encode_versioned_value for consumed bytes");
    let (decoded, ver, consumed): (FarmStockV1, Version, usize) =
        decode_versioned_value(&versioned_encoded)
            .expect("decode_versioned_value for consumed bytes");

    assert_eq!(decoded, stock);
    assert_eq!(ver, version);
    // consumed now includes the 11-byte versioned header
    let (_decoded_raw, raw_consumed): (FarmStockV1, usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice raw payload");
    let versioned_header_size = 11usize;
    assert_eq!(
        consumed,
        raw_consumed + versioned_header_size,
        "consumed bytes must equal raw payload consumed + header size"
    );
    // The versioned encoding must be strictly larger than the raw payload
    assert!(versioned_encoded.len() > raw_payload.len());
}

/// Test 22 — harvest price calculation roundtrip: total value preserved
#[test]
fn test_harvest_price_calculation_roundtrip() {
    let version = Version::new(1, 0, 0);
    // 3_000 kg at $12.50/kg = $37_500.00  (price stored as cents/kg = 1_250)
    let record = HarvestRecord {
        harvest_id: 50_001,
        farm_id: 400,
        species: FishSpecies::Salmon,
        quantity_kg: 3_000,
        price_cents_per_kg: 1_250,
        harvested_at: 1_740_000_000,
    };
    let total_cents_expected: u64 = record.quantity_kg as u64 * record.price_cents_per_kg as u64;
    assert_eq!(total_cents_expected, 3_750_000); // $37_500.00

    let encoded = encode_versioned_value(&record, version).expect("encode harvest price record");
    let (decoded, ver, _): (HarvestRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode harvest price record");

    assert_eq!(decoded, record);
    assert_eq!(ver, version);
    let total_cents_decoded: u64 = decoded.quantity_kg as u64 * decoded.price_cents_per_kg as u64;
    assert_eq!(
        total_cents_decoded, total_cents_expected,
        "harvest total value must survive versioned roundtrip"
    );
}
