//! Advanced nested structs test — wine production and viticulture management theme, 22 tests.

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
// Domain types — Fermentation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[allow(clippy::upper_case_acronyms)]
enum VesselType {
    StainlessSteel,
    Concrete,
    OpenTopWood,
    Amphora,
    HDPE,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FermentationReading {
    hours_elapsed: u32,
    temperature_c: f64,
    brix: f64,
    ph: f64,
    dissolved_o2_ppm: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct YeastAddition {
    strain_name: String,
    dosage_g_per_hl: f64,
    inoculation_hour: u32,
    rehydrated: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FermentationTank {
    tank_id: String,
    vessel: VesselType,
    capacity_liters: u32,
    lot_ids: Vec<String>,
    yeast: YeastAddition,
    readings: Vec<FermentationReading>,
    malolactic_started: bool,
    cold_soak_hours: Option<u32>,
}

// ---------------------------------------------------------------------------
// Domain types — Barrel Aging
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ToastLevel {
    Light,
    MediumMinus,
    Medium,
    MediumPlus,
    Heavy,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OakOrigin {
    FrenchAllier,
    FrenchTronçais,
    FrenchVosges,
    AmericanMissouri,
    HungarianZemplen,
    SlavonianCroatia,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Barrel {
    barrel_id: String,
    oak_origin: OakOrigin,
    toast_level: ToastLevel,
    volume_liters: u16,
    use_count: u8,
    cooperage: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BarrelSample {
    sample_date: String,
    free_so2_ppm: f64,
    total_so2_ppm: f64,
    va_g_per_l: f64,
    ph: f64,
    visual_clarity: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BarrelAgingProgram {
    program_name: String,
    target_months: u16,
    barrels: Vec<Barrel>,
    samples: Vec<BarrelSample>,
    racking_count: u8,
    topped_monthly: bool,
}

// ---------------------------------------------------------------------------
// Domain types — Blending
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BlendComponent {
    source_lot: String,
    variety: GrapeVariety,
    percentage: f64,
    barrel_id: Option<String>,
    vintage_year: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BlendTrial {
    trial_id: String,
    trial_date: String,
    components: Vec<BlendComponent>,
    taster_score: u8,
    selected: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BlendingSession {
    session_name: String,
    winemaker: String,
    target_wine: String,
    trials: Vec<BlendTrial>,
    final_selection: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Chemistry
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WineChemistry {
    sample_id: String,
    alcohol_pct: f64,
    ph: f64,
    titratable_acidity_g_per_l: f64,
    volatile_acidity_g_per_l: f64,
    free_so2_ppm: f64,
    total_so2_ppm: f64,
    residual_sugar_g_per_l: f64,
    malic_acid_g_per_l: f64,
    lactic_acid_g_per_l: f64,
    color_intensity: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LabPanel {
    lab_name: String,
    analysis_date: String,
    results: Vec<WineChemistry>,
    certified: bool,
}

// ---------------------------------------------------------------------------
// Domain types — Tasting Notes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AppearanceScore {
    clarity: u8,
    color_depth: u8,
    viscosity: u8,
    description: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NoseScore {
    intensity: u8,
    complexity: u8,
    fruit_character: String,
    secondary_aromas: Vec<String>,
    tertiary_aromas: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PalateScore {
    body: u8,
    tannin: u8,
    acidity: u8,
    finish_length_seconds: u16,
    balance: u8,
    flavor_descriptors: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TastingNote {
    taster_name: String,
    date: String,
    wine_label: String,
    vintage: u16,
    appearance: AppearanceScore,
    nose: NoseScore,
    palate: PalateScore,
    overall_score: u8,
    drink_window_start: u16,
    drink_window_end: u16,
}

// ---------------------------------------------------------------------------
// Domain types — Bottling
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ClosureType {
    NaturalCork,
    SyntheticCork,
    ScrewCap,
    GlassStopper,
    CrownCap,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QcSample {
    bottle_number: u32,
    fill_level_ml: f64,
    headspace_mm: f64,
    cork_compression_ok: bool,
    label_placement_ok: bool,
    capsule_intact: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BottlingRun {
    run_id: String,
    date: String,
    wine_lot: String,
    closure: ClosureType,
    bottle_count: u32,
    line_speed_per_hour: u16,
    qc_samples: Vec<QcSample>,
    final_so2_ppm: f64,
    filter_micron: f64,
}

// ---------------------------------------------------------------------------
// Domain types — Cellar & Inventory
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BinLocation {
    cellar_zone: String,
    row: u16,
    column: u16,
    level: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct InventoryEntry {
    wine_label: String,
    vintage: u16,
    format_ml: u16,
    quantity: u32,
    bin: BinLocation,
    arrival_date: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CellarInventory {
    cellar_name: String,
    temperature_c: f64,
    humidity_pct: f64,
    entries: Vec<InventoryEntry>,
    last_audit_date: String,
}

// ---------------------------------------------------------------------------
// Domain types — Appellation Compliance
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct VarietyRequirement {
    variety: GrapeVariety,
    min_pct: f64,
    max_pct: Option<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AppellationRule {
    appellation_name: String,
    country: String,
    region: String,
    max_yield_hl_per_ha: f64,
    min_alcohol_pct: f64,
    required_varieties: Vec<VarietyRequirement>,
    oak_aging_months_min: Option<u16>,
    allowed_irrigation: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ComplianceCheck {
    wine_lot: String,
    rule: AppellationRule,
    actual_yield_hl_per_ha: f64,
    actual_alcohol_pct: f64,
    variety_breakdown: Vec<BlendComponent>,
    passed: bool,
    inspector_name: String,
    inspection_date: String,
}

// ---------------------------------------------------------------------------
// Domain types — Full Winery
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WineProgram {
    wine_name: String,
    vintage: u16,
    harvest: VintageHarvest,
    fermentation_tanks: Vec<FermentationTank>,
    aging: BarrelAgingProgram,
    blending: BlendingSession,
    chemistry: LabPanel,
    tasting_notes: Vec<TastingNote>,
    bottling: Option<BottlingRun>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Winery {
    name: String,
    location: GpsCoord,
    blocks: Vec<VineyardBlock>,
    programs: Vec<WineProgram>,
    cellar: CellarInventory,
    compliance_records: Vec<ComplianceCheck>,
    established_year: u16,
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

fn make_reading(hours: u32, temp: f64, brix: f64, ph: f64) -> FermentationReading {
    FermentationReading {
        hours_elapsed: hours,
        temperature_c: temp,
        brix,
        ph,
        dissolved_o2_ppm: 0.3,
    }
}

fn make_barrel(id: &str, origin: OakOrigin, toast: ToastLevel, use_count: u8) -> Barrel {
    Barrel {
        barrel_id: id.to_string(),
        oak_origin: origin,
        toast_level: toast,
        volume_liters: 225,
        use_count,
        cooperage: "Seguin Moreau".to_string(),
    }
}

fn make_barrel_sample(date: &str, free_so2: f64, va: f64) -> BarrelSample {
    BarrelSample {
        sample_date: date.to_string(),
        free_so2_ppm: free_so2,
        total_so2_ppm: free_so2 * 2.5,
        va_g_per_l: va,
        ph: 3.62,
        visual_clarity: 8,
    }
}

fn make_chemistry(sample_id: &str, alcohol: f64, ph: f64) -> WineChemistry {
    WineChemistry {
        sample_id: sample_id.to_string(),
        alcohol_pct: alcohol,
        ph,
        titratable_acidity_g_per_l: 5.9,
        volatile_acidity_g_per_l: 0.42,
        free_so2_ppm: 28.0,
        total_so2_ppm: 78.0,
        residual_sugar_g_per_l: 1.2,
        malic_acid_g_per_l: 0.1,
        lactic_acid_g_per_l: 1.8,
        color_intensity: 12.5,
    }
}

fn make_tasting_note(taster: &str, wine: &str, vintage: u16, score: u8) -> TastingNote {
    TastingNote {
        taster_name: taster.to_string(),
        date: "2025-06-15".to_string(),
        wine_label: wine.to_string(),
        vintage,
        appearance: AppearanceScore {
            clarity: 9,
            color_depth: 8,
            viscosity: 7,
            description: "Deep garnet with purple rim".to_string(),
        },
        nose: NoseScore {
            intensity: 8,
            complexity: 9,
            fruit_character: "Blackcurrant and plum".to_string(),
            secondary_aromas: vec!["Cedar".to_string(), "Vanilla".to_string()],
            tertiary_aromas: vec!["Leather".to_string(), "Tobacco".to_string()],
        },
        palate: PalateScore {
            body: 8,
            tannin: 7,
            acidity: 7,
            finish_length_seconds: 45,
            balance: 9,
            flavor_descriptors: vec![
                "Cassis".to_string(),
                "Graphite".to_string(),
                "Dark chocolate".to_string(),
            ],
        },
        overall_score: score,
        drink_window_start: 2026,
        drink_window_end: 2040,
    }
}

fn make_qc_sample(bottle_num: u32) -> QcSample {
    QcSample {
        bottle_number: bottle_num,
        fill_level_ml: 750.2,
        headspace_mm: 63.0,
        cork_compression_ok: true,
        label_placement_ok: true,
        capsule_intact: true,
    }
}

fn make_bin(zone: &str, row: u16, col: u16, level: u8) -> BinLocation {
    BinLocation {
        cellar_zone: zone.to_string(),
        row,
        column: col,
        level,
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

// ---------------------------------------------------------------------------
// Test 7: Fermentation tank with readings curve
// ---------------------------------------------------------------------------
#[test]
fn test_fermentation_tank_monitoring() {
    let tank = FermentationTank {
        tank_id: "T-14".to_string(),
        vessel: VesselType::StainlessSteel,
        capacity_liters: 5000,
        lot_ids: vec!["LOT-001".to_string(), "LOT-002".to_string()],
        yeast: YeastAddition {
            strain_name: "RC212".to_string(),
            dosage_g_per_hl: 25.0,
            inoculation_hour: 48,
            rehydrated: true,
        },
        readings: vec![
            make_reading(0, 12.0, 24.5, 3.45),
            make_reading(24, 15.0, 23.0, 3.42),
            make_reading(48, 22.0, 18.5, 3.40),
            make_reading(72, 26.0, 12.0, 3.48),
            make_reading(96, 28.0, 6.5, 3.52),
            make_reading(120, 27.0, 2.0, 3.55),
            make_reading(144, 25.0, -1.0, 3.58),
        ],
        malolactic_started: false,
        cold_soak_hours: Some(48),
    };
    roundtrip(&tank, "fermentation tank with 7 readings");
}

// ---------------------------------------------------------------------------
// Test 8: Barrel aging program with French oak
// ---------------------------------------------------------------------------
#[test]
fn test_barrel_aging_program() {
    let program = BarrelAgingProgram {
        program_name: "Reserve Rouge 2024".to_string(),
        target_months: 18,
        barrels: vec![
            make_barrel("B-001", OakOrigin::FrenchAllier, ToastLevel::MediumPlus, 0),
            make_barrel("B-002", OakOrigin::FrenchTronçais, ToastLevel::Medium, 0),
            make_barrel("B-003", OakOrigin::FrenchAllier, ToastLevel::Heavy, 1),
            make_barrel("B-004", OakOrigin::HungarianZemplen, ToastLevel::Medium, 2),
        ],
        samples: vec![
            make_barrel_sample("2024-12-15", 32.0, 0.35),
            make_barrel_sample("2025-03-15", 28.0, 0.38),
            make_barrel_sample("2025-06-15", 25.0, 0.40),
        ],
        racking_count: 2,
        topped_monthly: true,
    };
    roundtrip(&program, "barrel aging program with 4 barrels");
}

// ---------------------------------------------------------------------------
// Test 9: Blending trial with component percentages
// ---------------------------------------------------------------------------
#[test]
fn test_blending_trial_components() {
    let trial = BlendTrial {
        trial_id: "BT-2024-007".to_string(),
        trial_date: "2025-03-10".to_string(),
        components: vec![
            BlendComponent {
                source_lot: "LOT-CS-01".to_string(),
                variety: GrapeVariety::CabernetSauvignon,
                percentage: 65.0,
                barrel_id: Some("B-001".to_string()),
                vintage_year: 2024,
            },
            BlendComponent {
                source_lot: "LOT-ME-01".to_string(),
                variety: GrapeVariety::Merlot,
                percentage: 25.0,
                barrel_id: Some("B-005".to_string()),
                vintage_year: 2024,
            },
            BlendComponent {
                source_lot: "LOT-SY-01".to_string(),
                variety: GrapeVariety::Syrah,
                percentage: 10.0,
                barrel_id: None,
                vintage_year: 2024,
            },
        ],
        taster_score: 92,
        selected: true,
        notes: "Excellent structure with silky tannins".to_string(),
    };
    roundtrip(&trial, "blend trial with 3 components");
}

// ---------------------------------------------------------------------------
// Test 10: Blending session with multiple trials
// ---------------------------------------------------------------------------
#[test]
fn test_blending_session_multiple_trials() {
    let session = BlendingSession {
        session_name: "Grand Vin 2024 Assembly".to_string(),
        winemaker: "Jean-Philippe Delmas".to_string(),
        target_wine: "Chateau Haut-Brion 2024".to_string(),
        trials: vec![
            BlendTrial {
                trial_id: "BT-001".to_string(),
                trial_date: "2025-02-20".to_string(),
                components: vec![
                    BlendComponent {
                        source_lot: "CS-A".to_string(),
                        variety: GrapeVariety::CabernetSauvignon,
                        percentage: 55.0,
                        barrel_id: Some("B-100".to_string()),
                        vintage_year: 2024,
                    },
                    BlendComponent {
                        source_lot: "ME-A".to_string(),
                        variety: GrapeVariety::Merlot,
                        percentage: 45.0,
                        barrel_id: Some("B-200".to_string()),
                        vintage_year: 2024,
                    },
                ],
                taster_score: 88,
                selected: false,
                notes: "Needs more structure".to_string(),
            },
            BlendTrial {
                trial_id: "BT-002".to_string(),
                trial_date: "2025-02-20".to_string(),
                components: vec![
                    BlendComponent {
                        source_lot: "CS-A".to_string(),
                        variety: GrapeVariety::CabernetSauvignon,
                        percentage: 70.0,
                        barrel_id: Some("B-100".to_string()),
                        vintage_year: 2024,
                    },
                    BlendComponent {
                        source_lot: "ME-A".to_string(),
                        variety: GrapeVariety::Merlot,
                        percentage: 30.0,
                        barrel_id: Some("B-200".to_string()),
                        vintage_year: 2024,
                    },
                ],
                taster_score: 94,
                selected: true,
                notes: "Beautiful balance and length".to_string(),
            },
        ],
        final_selection: Some("BT-002".to_string()),
    };
    roundtrip(&session, "blending session with 2 trials");
}

// ---------------------------------------------------------------------------
// Test 11: Wine chemistry panel
// ---------------------------------------------------------------------------
#[test]
fn test_wine_chemistry_lab_panel() {
    let panel = LabPanel {
        lab_name: "ETS Laboratories".to_string(),
        analysis_date: "2025-04-01".to_string(),
        results: vec![
            make_chemistry("CHEM-001", 13.5, 3.62),
            make_chemistry("CHEM-002", 14.2, 3.55),
            make_chemistry("CHEM-003", 12.8, 3.70),
        ],
        certified: true,
    };
    roundtrip(&panel, "lab panel with 3 chemistry results");
}

// ---------------------------------------------------------------------------
// Test 12: Detailed tasting note with appearance/nose/palate
// ---------------------------------------------------------------------------
#[test]
fn test_structured_tasting_note() {
    let note = TastingNote {
        taster_name: "Jancis Robinson".to_string(),
        date: "2025-11-01".to_string(),
        wine_label: "Domaine de la Romanee-Conti".to_string(),
        vintage: 2021,
        appearance: AppearanceScore {
            clarity: 10,
            color_depth: 6,
            viscosity: 5,
            description: "Pale ruby with brick edge, brilliant clarity".to_string(),
        },
        nose: NoseScore {
            intensity: 9,
            complexity: 10,
            fruit_character: "Wild strawberry and rose petal".to_string(),
            secondary_aromas: vec![
                "Cinnamon".to_string(),
                "Clove".to_string(),
                "New oak".to_string(),
            ],
            tertiary_aromas: vec![
                "Truffle".to_string(),
                "Forest floor".to_string(),
                "Dried herbs".to_string(),
                "Iron".to_string(),
            ],
        },
        palate: PalateScore {
            body: 6,
            tannin: 5,
            acidity: 8,
            finish_length_seconds: 90,
            balance: 10,
            flavor_descriptors: vec![
                "Cherry".to_string(),
                "Spice".to_string(),
                "Mineral".to_string(),
                "Silk".to_string(),
                "Earth".to_string(),
            ],
        },
        overall_score: 99,
        drink_window_start: 2025,
        drink_window_end: 2060,
    };
    roundtrip(&note, "DRC tasting note");
}

// ---------------------------------------------------------------------------
// Test 13: Bottling run with QC samples
// ---------------------------------------------------------------------------
#[test]
fn test_bottling_run_qc_samples() {
    let run = BottlingRun {
        run_id: "BTL-2025-042".to_string(),
        date: "2025-09-15".to_string(),
        wine_lot: "LOT-GV-2024".to_string(),
        closure: ClosureType::NaturalCork,
        bottle_count: 12000,
        line_speed_per_hour: 2400,
        qc_samples: vec![
            make_qc_sample(1),
            make_qc_sample(3000),
            make_qc_sample(6000),
            make_qc_sample(9000),
            make_qc_sample(12000),
        ],
        final_so2_ppm: 30.0,
        filter_micron: 0.45,
    };
    roundtrip(&run, "bottling run with 5 QC samples");
}

// ---------------------------------------------------------------------------
// Test 14: Cellar inventory with bin locations
// ---------------------------------------------------------------------------
#[test]
fn test_cellar_inventory_bin_locations() {
    let inv = CellarInventory {
        cellar_name: "Cave Principale".to_string(),
        temperature_c: 13.5,
        humidity_pct: 72.0,
        entries: vec![
            InventoryEntry {
                wine_label: "Grand Vin 2020".to_string(),
                vintage: 2020,
                format_ml: 750,
                quantity: 240,
                bin: make_bin("A", 1, 3, 2),
                arrival_date: "2022-06-01".to_string(),
            },
            InventoryEntry {
                wine_label: "Grand Vin 2020".to_string(),
                vintage: 2020,
                format_ml: 1500,
                quantity: 48,
                bin: make_bin("A", 1, 4, 1),
                arrival_date: "2022-06-01".to_string(),
            },
            InventoryEntry {
                wine_label: "Second Vin 2021".to_string(),
                vintage: 2021,
                format_ml: 750,
                quantity: 600,
                bin: make_bin("B", 3, 1, 1),
                arrival_date: "2023-03-15".to_string(),
            },
            InventoryEntry {
                wine_label: "Rose 2023".to_string(),
                vintage: 2023,
                format_ml: 750,
                quantity: 120,
                bin: make_bin("C", 1, 1, 1),
                arrival_date: "2024-01-10".to_string(),
            },
        ],
        last_audit_date: "2025-01-15".to_string(),
    };
    roundtrip(&inv, "cellar inventory with 4 entries");
}

// ---------------------------------------------------------------------------
// Test 15: Appellation compliance check
// ---------------------------------------------------------------------------
#[test]
fn test_appellation_compliance_check() {
    let check = ComplianceCheck {
        wine_lot: "LOT-GV-2024".to_string(),
        rule: AppellationRule {
            appellation_name: "Saint-Emilion Grand Cru".to_string(),
            country: "France".to_string(),
            region: "Bordeaux".to_string(),
            max_yield_hl_per_ha: 46.0,
            min_alcohol_pct: 11.5,
            required_varieties: vec![
                VarietyRequirement {
                    variety: GrapeVariety::Merlot,
                    min_pct: 0.0,
                    max_pct: None,
                },
                VarietyRequirement {
                    variety: GrapeVariety::CabernetSauvignon,
                    min_pct: 0.0,
                    max_pct: None,
                },
            ],
            oak_aging_months_min: Some(12),
            allowed_irrigation: false,
        },
        actual_yield_hl_per_ha: 38.5,
        actual_alcohol_pct: 13.8,
        variety_breakdown: vec![
            BlendComponent {
                source_lot: "ME-01".to_string(),
                variety: GrapeVariety::Merlot,
                percentage: 80.0,
                barrel_id: None,
                vintage_year: 2024,
            },
            BlendComponent {
                source_lot: "CS-01".to_string(),
                variety: GrapeVariety::CabernetSauvignon,
                percentage: 20.0,
                barrel_id: None,
                vintage_year: 2024,
            },
        ],
        passed: true,
        inspector_name: "Pierre Dupont".to_string(),
        inspection_date: "2025-07-01".to_string(),
    };
    roundtrip(&check, "appellation compliance check");
}

// ---------------------------------------------------------------------------
// Test 16: Fermentation with amphora vessel and no cold soak
// ---------------------------------------------------------------------------
#[test]
fn test_amphora_fermentation_no_cold_soak() {
    let tank = FermentationTank {
        tank_id: "AMP-03".to_string(),
        vessel: VesselType::Amphora,
        capacity_liters: 800,
        lot_ids: vec!["LOT-GR-01".to_string()],
        yeast: YeastAddition {
            strain_name: "Indigenous".to_string(),
            dosage_g_per_hl: 0.0,
            inoculation_hour: 0,
            rehydrated: false,
        },
        readings: vec![
            make_reading(0, 18.0, 25.0, 3.50),
            make_reading(48, 20.0, 22.0, 3.48),
            make_reading(96, 23.0, 15.0, 3.52),
            make_reading(168, 21.0, 5.0, 3.58),
            make_reading(240, 19.0, 0.0, 3.62),
        ],
        malolactic_started: true,
        cold_soak_hours: None,
    };
    roundtrip(&tank, "amphora fermentation without cold soak");
}

// ---------------------------------------------------------------------------
// Test 17: Multiple tasting notes for the same wine
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_tasting_notes_same_wine() {
    let notes = [
        make_tasting_note("Robert Parker", "Opus One", 2021, 96),
        make_tasting_note("James Suckling", "Opus One", 2021, 98),
        make_tasting_note("Antonio Galloni", "Opus One", 2021, 95),
        make_tasting_note("Neal Martin", "Opus One", 2021, 94),
    ];
    for (i, note) in notes.iter().enumerate() {
        roundtrip(note, &format!("tasting note #{}", i + 1));
    }
}

// ---------------------------------------------------------------------------
// Test 18: Bottling run with screw cap closure
// ---------------------------------------------------------------------------
#[test]
fn test_bottling_screwcap_roundtrip() {
    let run = BottlingRun {
        run_id: "BTL-2025-SB-001".to_string(),
        date: "2025-06-20".to_string(),
        wine_lot: "LOT-SB-2024".to_string(),
        closure: ClosureType::ScrewCap,
        bottle_count: 24000,
        line_speed_per_hour: 3600,
        qc_samples: vec![
            QcSample {
                bottle_number: 500,
                fill_level_ml: 749.8,
                headspace_mm: 64.0,
                cork_compression_ok: true,
                label_placement_ok: true,
                capsule_intact: true,
            },
            QcSample {
                bottle_number: 12000,
                fill_level_ml: 750.1,
                headspace_mm: 63.5,
                cork_compression_ok: true,
                label_placement_ok: false,
                capsule_intact: true,
            },
        ],
        final_so2_ppm: 35.0,
        filter_micron: 0.20,
    };
    roundtrip(&run, "screwcap bottling run with label defect");
}

// ---------------------------------------------------------------------------
// Test 19: Deeply nested wine program
// ---------------------------------------------------------------------------
#[test]
fn test_full_wine_program_deep_nesting() {
    let program = WineProgram {
        wine_name: "Cuvee Prestige".to_string(),
        vintage: 2024,
        harvest: VintageHarvest {
            vintage_year: 2024,
            vineyard_block: "Cote Rotie - La Landonne".to_string(),
            harvest_days: vec![HarvestDay {
                date_iso: "2024-09-20".to_string(),
                crew_size: 30,
                start_hour: 5,
                end_hour: 13,
                temperature_c: 15.0,
                lots: vec![
                    make_grape_lot("SY-LAND-01", GrapeVariety::Syrah, 24.8),
                    make_grape_lot("SY-LAND-02", GrapeVariety::Syrah, 25.1),
                ],
            }],
            total_yield_tonnes: 12.0,
            notes: "Cool morning harvest, pristine fruit".to_string(),
        },
        fermentation_tanks: vec![FermentationTank {
            tank_id: "T-22".to_string(),
            vessel: VesselType::OpenTopWood,
            capacity_liters: 3000,
            lot_ids: vec!["SY-LAND-01".to_string(), "SY-LAND-02".to_string()],
            yeast: YeastAddition {
                strain_name: "D254".to_string(),
                dosage_g_per_hl: 20.0,
                inoculation_hour: 72,
                rehydrated: true,
            },
            readings: vec![
                make_reading(0, 10.0, 25.0, 3.40),
                make_reading(72, 28.0, 14.0, 3.50),
                make_reading(144, 25.0, 0.0, 3.58),
            ],
            malolactic_started: true,
            cold_soak_hours: Some(72),
        }],
        aging: BarrelAgingProgram {
            program_name: "Prestige 36-month".to_string(),
            target_months: 36,
            barrels: vec![
                make_barrel("P-001", OakOrigin::FrenchAllier, ToastLevel::MediumPlus, 0),
                make_barrel("P-002", OakOrigin::FrenchVosges, ToastLevel::Medium, 0),
            ],
            samples: vec![
                make_barrel_sample("2025-01-15", 30.0, 0.30),
                make_barrel_sample("2025-07-15", 26.0, 0.35),
            ],
            racking_count: 3,
            topped_monthly: true,
        },
        blending: BlendingSession {
            session_name: "Prestige Assembly".to_string(),
            winemaker: "Stephane Ogier".to_string(),
            target_wine: "Cuvee Prestige".to_string(),
            trials: vec![BlendTrial {
                trial_id: "PT-001".to_string(),
                trial_date: "2027-01-10".to_string(),
                components: vec![BlendComponent {
                    source_lot: "SY-LAND-01".to_string(),
                    variety: GrapeVariety::Syrah,
                    percentage: 100.0,
                    barrel_id: Some("P-001".to_string()),
                    vintage_year: 2024,
                }],
                taster_score: 97,
                selected: true,
                notes: "Pure expression of terroir".to_string(),
            }],
            final_selection: Some("PT-001".to_string()),
        },
        chemistry: LabPanel {
            lab_name: "Rhone Valley Lab".to_string(),
            analysis_date: "2027-02-01".to_string(),
            results: vec![make_chemistry("PREST-001", 13.2, 3.55)],
            certified: true,
        },
        tasting_notes: vec![make_tasting_note(
            "Michel Bettane",
            "Cuvee Prestige",
            2024,
            97,
        )],
        bottling: Some(BottlingRun {
            run_id: "BTL-PREST-2027".to_string(),
            date: "2027-06-01".to_string(),
            wine_lot: "PREST-2024".to_string(),
            closure: ClosureType::NaturalCork,
            bottle_count: 3600,
            line_speed_per_hour: 1200,
            qc_samples: vec![
                make_qc_sample(1),
                make_qc_sample(1800),
                make_qc_sample(3600),
            ],
            final_so2_ppm: 25.0,
            filter_micron: 0.65,
        }),
    };
    roundtrip(&program, "full wine program deeply nested");
}

// ---------------------------------------------------------------------------
// Test 20: Wine program with no bottling yet
// ---------------------------------------------------------------------------
#[test]
fn test_wine_program_unbottled() {
    let program = WineProgram {
        wine_name: "Village Blanc".to_string(),
        vintage: 2025,
        harvest: VintageHarvest {
            vintage_year: 2025,
            vineyard_block: "Les Perrieres".to_string(),
            harvest_days: vec![HarvestDay {
                date_iso: "2025-09-10".to_string(),
                crew_size: 12,
                start_hour: 6,
                end_hour: 10,
                temperature_c: 12.0,
                lots: vec![make_grape_lot("CH-PER-01", GrapeVariety::Chardonnay, 22.0)],
            }],
            total_yield_tonnes: 5.5,
            notes: "Early harvest for freshness".to_string(),
        },
        fermentation_tanks: vec![FermentationTank {
            tank_id: "T-W05".to_string(),
            vessel: VesselType::StainlessSteel,
            capacity_liters: 2000,
            lot_ids: vec!["CH-PER-01".to_string()],
            yeast: YeastAddition {
                strain_name: "CY3079".to_string(),
                dosage_g_per_hl: 20.0,
                inoculation_hour: 24,
                rehydrated: true,
            },
            readings: vec![
                make_reading(0, 14.0, 22.0, 3.30),
                make_reading(48, 16.0, 18.0, 3.28),
                make_reading(120, 16.0, 4.0, 3.35),
            ],
            malolactic_started: false,
            cold_soak_hours: None,
        }],
        aging: BarrelAgingProgram {
            program_name: "Blanc sur lie".to_string(),
            target_months: 10,
            barrels: vec![make_barrel(
                "W-001",
                OakOrigin::FrenchAllier,
                ToastLevel::Light,
                2,
            )],
            samples: vec![make_barrel_sample("2026-01-15", 35.0, 0.28)],
            racking_count: 1,
            topped_monthly: true,
        },
        blending: BlendingSession {
            session_name: "Blanc 2025".to_string(),
            winemaker: "Dominique Lafon".to_string(),
            target_wine: "Village Blanc".to_string(),
            trials: vec![],
            final_selection: None,
        },
        chemistry: LabPanel {
            lab_name: "Beaune Lab".to_string(),
            analysis_date: "2026-05-01".to_string(),
            results: vec![make_chemistry("BL-001", 12.5, 3.28)],
            certified: false,
        },
        tasting_notes: vec![],
        bottling: None,
    };
    roundtrip(&program, "unbottled wine program with None bottling");
}

// ---------------------------------------------------------------------------
// Test 21: Full winery with everything
// ---------------------------------------------------------------------------
#[test]
fn test_full_winery_deeply_nested() {
    let winery = Winery {
        name: "Domaine de la Cote d'Or".to_string(),
        location: make_coord(47.025, 4.842, 280.0),
        blocks: vec![VineyardBlock {
            block_name: "Premier Cru Les Amoureuses".to_string(),
            appellation: "Chambolle-Musigny".to_string(),
            zones: vec![
                make_zone("CM-01", SoilType::Limestone, "3309C"),
                make_zone("CM-02", SoilType::Clay, "SO4"),
            ],
            total_hectares: 2.8,
            certified_organic: true,
        }],
        programs: vec![WineProgram {
            wine_name: "Les Amoureuses".to_string(),
            vintage: 2023,
            harvest: VintageHarvest {
                vintage_year: 2023,
                vineyard_block: "Les Amoureuses".to_string(),
                harvest_days: vec![HarvestDay {
                    date_iso: "2023-09-18".to_string(),
                    crew_size: 20,
                    start_hour: 5,
                    end_hour: 11,
                    temperature_c: 14.0,
                    lots: vec![make_grape_lot("PN-AM-01", GrapeVariety::PinotNoir, 23.5)],
                }],
                total_yield_tonnes: 8.0,
                notes: "Small but concentrated crop".to_string(),
            },
            fermentation_tanks: vec![FermentationTank {
                tank_id: "T-PC01".to_string(),
                vessel: VesselType::OpenTopWood,
                capacity_liters: 2500,
                lot_ids: vec!["PN-AM-01".to_string()],
                yeast: YeastAddition {
                    strain_name: "Indigenous".to_string(),
                    dosage_g_per_hl: 0.0,
                    inoculation_hour: 0,
                    rehydrated: false,
                },
                readings: vec![
                    make_reading(0, 12.0, 23.5, 3.38),
                    make_reading(120, 30.0, 6.0, 3.50),
                    make_reading(216, 24.0, 0.0, 3.56),
                ],
                malolactic_started: true,
                cold_soak_hours: Some(96),
            }],
            aging: BarrelAgingProgram {
                program_name: "1er Cru 18-month".to_string(),
                target_months: 18,
                barrels: vec![
                    make_barrel(
                        "AM-001",
                        OakOrigin::FrenchVosges,
                        ToastLevel::MediumMinus,
                        0,
                    ),
                    make_barrel("AM-002", OakOrigin::FrenchAllier, ToastLevel::Medium, 1),
                ],
                samples: vec![
                    make_barrel_sample("2024-03-01", 28.0, 0.32),
                    make_barrel_sample("2024-09-01", 24.0, 0.36),
                ],
                racking_count: 2,
                topped_monthly: true,
            },
            blending: BlendingSession {
                session_name: "Amoureuses 2023 Assembly".to_string(),
                winemaker: "Christophe Roumier".to_string(),
                target_wine: "Les Amoureuses".to_string(),
                trials: vec![BlendTrial {
                    trial_id: "AM-T1".to_string(),
                    trial_date: "2025-02-15".to_string(),
                    components: vec![BlendComponent {
                        source_lot: "PN-AM-01".to_string(),
                        variety: GrapeVariety::PinotNoir,
                        percentage: 100.0,
                        barrel_id: None,
                        vintage_year: 2023,
                    }],
                    taster_score: 96,
                    selected: true,
                    notes: "Ethereal, layered, profound".to_string(),
                }],
                final_selection: Some("AM-T1".to_string()),
            },
            chemistry: LabPanel {
                lab_name: "Lab Burgundy".to_string(),
                analysis_date: "2025-03-01".to_string(),
                results: vec![make_chemistry("AM-CHEM-01", 13.0, 3.52)],
                certified: true,
            },
            tasting_notes: vec![make_tasting_note(
                "Allen Meadows",
                "Les Amoureuses",
                2023,
                96,
            )],
            bottling: Some(BottlingRun {
                run_id: "BTL-AM-2025".to_string(),
                date: "2025-04-15".to_string(),
                wine_lot: "AM-2023".to_string(),
                closure: ClosureType::NaturalCork,
                bottle_count: 4800,
                line_speed_per_hour: 1000,
                qc_samples: vec![make_qc_sample(1), make_qc_sample(4800)],
                final_so2_ppm: 22.0,
                filter_micron: 0.0,
            }),
        }],
        cellar: CellarInventory {
            cellar_name: "Cave Historique".to_string(),
            temperature_c: 12.0,
            humidity_pct: 78.0,
            entries: vec![
                InventoryEntry {
                    wine_label: "Les Amoureuses 2022".to_string(),
                    vintage: 2022,
                    format_ml: 750,
                    quantity: 180,
                    bin: make_bin("Premier", 1, 1, 1),
                    arrival_date: "2024-05-01".to_string(),
                },
                InventoryEntry {
                    wine_label: "Les Amoureuses 2021".to_string(),
                    vintage: 2021,
                    format_ml: 1500,
                    quantity: 36,
                    bin: make_bin("Premier", 1, 2, 1),
                    arrival_date: "2023-05-01".to_string(),
                },
            ],
            last_audit_date: "2025-12-01".to_string(),
        },
        compliance_records: vec![ComplianceCheck {
            wine_lot: "AM-2023".to_string(),
            rule: AppellationRule {
                appellation_name: "Chambolle-Musigny 1er Cru".to_string(),
                country: "France".to_string(),
                region: "Burgundy".to_string(),
                max_yield_hl_per_ha: 40.0,
                min_alcohol_pct: 11.0,
                required_varieties: vec![VarietyRequirement {
                    variety: GrapeVariety::PinotNoir,
                    min_pct: 100.0,
                    max_pct: Some(100.0),
                }],
                oak_aging_months_min: None,
                allowed_irrigation: false,
            },
            actual_yield_hl_per_ha: 28.5,
            actual_alcohol_pct: 13.0,
            variety_breakdown: vec![BlendComponent {
                source_lot: "PN-AM-01".to_string(),
                variety: GrapeVariety::PinotNoir,
                percentage: 100.0,
                barrel_id: None,
                vintage_year: 2023,
            }],
            passed: true,
            inspector_name: "Marie Leclerc".to_string(),
            inspection_date: "2025-06-01".to_string(),
        }],
        established_year: 1720,
    };
    roundtrip(&winery, "full winery deeply nested");
}

// ---------------------------------------------------------------------------
// Test 22: Empty collections and optional fields throughout nesting
// ---------------------------------------------------------------------------
#[test]
fn test_edge_case_empty_collections_and_optionals() {
    let block = VineyardBlock {
        block_name: "New Planting".to_string(),
        appellation: "Pending Classification".to_string(),
        zones: vec![],
        total_hectares: 0.0,
        certified_organic: false,
    };
    roundtrip(&block, "vineyard block with zero zones");

    let harvest = VintageHarvest {
        vintage_year: 2025,
        vineyard_block: "Dormant Block".to_string(),
        harvest_days: vec![],
        total_yield_tonnes: 0.0,
        notes: String::new(),
    };
    roundtrip(&harvest, "vintage with no harvest days");

    let session = BlendingSession {
        session_name: "Early Planning".to_string(),
        winemaker: String::new(),
        target_wine: "TBD".to_string(),
        trials: vec![],
        final_selection: None,
    };
    roundtrip(&session, "blending session with no trials");

    let cellar = CellarInventory {
        cellar_name: "New Facility".to_string(),
        temperature_c: 14.0,
        humidity_pct: 65.0,
        entries: vec![],
        last_audit_date: "2025-01-01".to_string(),
    };
    roundtrip(&cellar, "empty cellar inventory");

    let panel = LabPanel {
        lab_name: "Awaiting Results".to_string(),
        analysis_date: String::new(),
        results: vec![],
        certified: false,
    };
    roundtrip(&panel, "lab panel with no results");
}
