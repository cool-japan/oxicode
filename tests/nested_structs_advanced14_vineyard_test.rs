//! Vineyard and harvest-focused tests for nested_structs_advanced14 (split from nested_structs_advanced14_test.rs).

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — Vineyard & Soil
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct GpsCoord {
    latitude: f64,
    longitude: f64,
    elevation_m: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SoilType {
    Clay,
    Limestone,
    Gravel,
    Schist,
    Volcanic,
    Loam,
    Sand,
    Chalk,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SoilAnalysis {
    soil_type: SoilType,
    ph: f64,
    organic_matter_pct: f64,
    nitrogen_ppm: f64,
    phosphorus_ppm: f64,
    potassium_ppm: f64,
    calcium_ppm: f64,
    depth_cm: u32,
    drainage_rating: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VineyardZone {
    zone_id: String,
    area_hectares: f64,
    soil: SoilAnalysis,
    coord: GpsCoord,
    rootstock: String,
    vine_age_years: u16,
    vine_density_per_ha: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VineyardBlock {
    block_name: String,
    appellation: String,
    zones: Vec<VineyardZone>,
    total_hectares: f64,
    certified_organic: bool,
}

// ---------------------------------------------------------------------------
// Domain types — Grape & Harvest
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum GrapeVariety {
    CabernetSauvignon,
    Merlot,
    PinotNoir,
    Chardonnay,
    SauvignonBlanc,
    Riesling,
    Syrah,
    Grenache,
    Tempranillo,
    Nebbiolo,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GrapeLot {
    lot_id: String,
    variety: GrapeVariety,
    source_zone: String,
    weight_kg: f64,
    brix_at_harvest: f64,
    ph_at_harvest: f64,
    ta_at_harvest: f64,
    hand_picked: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HarvestDay {
    date_iso: String,
    crew_size: u16,
    start_hour: u8,
    end_hour: u8,
    temperature_c: f64,
    lots: Vec<GrapeLot>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VintageHarvest {
    vintage_year: u16,
    vineyard_block: String,
    harvest_days: Vec<HarvestDay>,
    total_yield_tonnes: f64,
    notes: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_coord(lat: f64, lon: f64, elev: f64) -> GpsCoord {
    GpsCoord {
        latitude: lat,
        longitude: lon,
        elevation_m: elev,
    }
}

fn make_soil(soil_type: SoilType, ph: f64) -> SoilAnalysis {
    SoilAnalysis {
        soil_type,
        ph,
        organic_matter_pct: 2.8,
        nitrogen_ppm: 45.0,
        phosphorus_ppm: 22.0,
        potassium_ppm: 180.0,
        calcium_ppm: 3200.0,
        depth_cm: 90,
        drainage_rating: 7,
    }
}

fn make_zone(id: &str, soil_type: SoilType, rootstock: &str) -> VineyardZone {
    VineyardZone {
        zone_id: id.to_string(),
        area_hectares: 1.5,
        soil: make_soil(soil_type, 7.2),
        coord: make_coord(44.83, -0.57, 35.0),
        rootstock: rootstock.to_string(),
        vine_age_years: 30,
        vine_density_per_ha: 6500,
    }
}

fn make_grape_lot(lot_id: &str, variety: GrapeVariety, brix: f64) -> GrapeLot {
    GrapeLot {
        lot_id: lot_id.to_string(),
        variety,
        source_zone: "Z-01".to_string(),
        weight_kg: 850.0,
        brix_at_harvest: brix,
        ph_at_harvest: 3.45,
        ta_at_harvest: 6.8,
        hand_picked: true,
    }
}

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {}", ctx));
    let (decoded, consumed): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {}", ctx));
    assert_eq!(val, &decoded, "roundtrip mismatch for {}", ctx);
    assert_eq!(consumed, bytes.len(), "byte count mismatch for {}", ctx);
}

// ---------------------------------------------------------------------------
// Test 1: Soil analysis roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_soil_analysis_roundtrip() {
    let soil = SoilAnalysis {
        soil_type: SoilType::Limestone,
        ph: 8.1,
        organic_matter_pct: 3.2,
        nitrogen_ppm: 52.0,
        phosphorus_ppm: 18.0,
        potassium_ppm: 210.0,
        calcium_ppm: 4500.0,
        depth_cm: 120,
        drainage_rating: 9,
    };
    roundtrip(&soil, "limestone soil analysis");
}

// ---------------------------------------------------------------------------
// Test 2: Vineyard zone with nested soil and coordinates
// ---------------------------------------------------------------------------
#[test]
fn test_vineyard_zone_nested_roundtrip() {
    let zone = make_zone("Z-Margaux-01", SoilType::Gravel, "SO4");
    roundtrip(&zone, "vineyard zone with nested soil");
}

// ---------------------------------------------------------------------------
// Test 3: Vineyard block with multiple zones
// ---------------------------------------------------------------------------
#[test]
fn test_vineyard_block_multiple_zones() {
    let block = VineyardBlock {
        block_name: "Grand Enclos".to_string(),
        appellation: "Pessac-Leognan".to_string(),
        zones: vec![
            make_zone("Z-01", SoilType::Gravel, "SO4"),
            make_zone("Z-02", SoilType::Clay, "3309C"),
            make_zone("Z-03", SoilType::Limestone, "101-14"),
        ],
        total_hectares: 4.5,
        certified_organic: true,
    };
    roundtrip(&block, "vineyard block with 3 zones");
}

// ---------------------------------------------------------------------------
// Test 4: Grape lot tracking with variety enum
// ---------------------------------------------------------------------------
#[test]
fn test_grape_lot_variety_tracking() {
    let lot = make_grape_lot("LOT-2024-CS-001", GrapeVariety::CabernetSauvignon, 24.5);
    roundtrip(&lot, "cabernet sauvignon grape lot");

    let lot2 = make_grape_lot("LOT-2024-PN-001", GrapeVariety::PinotNoir, 23.2);
    roundtrip(&lot2, "pinot noir grape lot");
}

// ---------------------------------------------------------------------------
// Test 5: Harvest day with multiple grape lots
// ---------------------------------------------------------------------------
#[test]
fn test_harvest_day_multiple_lots() {
    let day = HarvestDay {
        date_iso: "2024-09-28".to_string(),
        crew_size: 24,
        start_hour: 6,
        end_hour: 14,
        temperature_c: 18.5,
        lots: vec![
            make_grape_lot("LOT-001", GrapeVariety::Merlot, 24.0),
            make_grape_lot("LOT-002", GrapeVariety::Merlot, 23.8),
            make_grape_lot("LOT-003", GrapeVariety::CabernetSauvignon, 24.5),
        ],
    };
    roundtrip(&day, "harvest day with 3 lots");
}

// ---------------------------------------------------------------------------
// Test 6: Vintage harvest with multiple harvest days
// ---------------------------------------------------------------------------
#[test]
fn test_vintage_harvest_record() {
    let harvest = VintageHarvest {
        vintage_year: 2024,
        vineyard_block: "Clos des Papes".to_string(),
        harvest_days: vec![
            HarvestDay {
                date_iso: "2024-09-25".to_string(),
                crew_size: 18,
                start_hour: 5,
                end_hour: 12,
                temperature_c: 16.0,
                lots: vec![make_grape_lot("V24-G-01", GrapeVariety::Grenache, 25.0)],
            },
            HarvestDay {
                date_iso: "2024-10-02".to_string(),
                crew_size: 22,
                start_hour: 6,
                end_hour: 15,
                temperature_c: 20.0,
                lots: vec![
                    make_grape_lot("V24-S-01", GrapeVariety::Syrah, 24.2),
                    make_grape_lot("V24-S-02", GrapeVariety::Syrah, 24.8),
                ],
            },
        ],
        total_yield_tonnes: 28.5,
        notes: "Exceptional vintage, dry September".to_string(),
    };
    roundtrip(&harvest, "vintage harvest record");
}
