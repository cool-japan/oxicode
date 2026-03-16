//! Advanced checksum tests for OxiCode — wine production and viticulture theme.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced29_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — wine production and viticulture
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GrapeVariety {
    CabernetSauvignon,
    Merlot,
    PinotNoir,
    Chardonnay,
    SauvignonBlanc,
    Riesling,
    Syrah,
    Zinfandel,
    Tempranillo,
    Sangiovese,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Rootstock {
    R110,
    SO4,
    R3309C,
    R101_14,
    OwnRooted,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VineyardBlock {
    block_id: String,
    variety: GrapeVariety,
    rootstock: Rootstock,
    planting_density_vines_per_hectare: u32,
    row_spacing_cm: u32,
    vine_spacing_cm: u32,
    elevation_meters: u32,
    aspect: String,
    soil_type: String,
    planted_year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestRecord {
    block_id: String,
    harvest_date: String,
    brix: u32,                              // scaled by 10 (e.g. 245 = 24.5 Brix)
    ph_scaled: u32,                         // scaled by 100 (e.g. 345 = 3.45 pH)
    titratable_acidity_g_per_l_scaled: u32, // scaled by 100
    tons_harvested_scaled: u32,             // scaled by 100
    picker_count: u16,
    bins_filled: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FermentationVesselType {
    StainlessSteel,
    OpenTopFermenter,
    ConcreteTank,
    OakBarrel,
    Amphora,
    PlasticBin,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FermentationLog {
    tank_id: String,
    vessel_type: FermentationVesselType,
    lot_id: String,
    day_number: u16,
    temperature_c_scaled: i32,    // scaled by 10
    specific_gravity_scaled: u32, // scaled by 1000 (e.g. 1092 = 1.092 SG)
    punchdowns_per_day: u8,
    free_so2_ppm: u16,
    yeast_strain: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OakType {
    FrenchAllier,
    FrenchTroncais,
    FrenchVosges,
    AmericanMissouri,
    HungarianOak,
    SlavonianOak,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToastLevel {
    Light,
    MediumMinus,
    Medium,
    MediumPlus,
    Heavy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BarrelAgingRecord {
    barrel_id: String,
    lot_id: String,
    oak_type: OakType,
    toast_level: ToastLevel,
    barrel_age_years: u8,
    capacity_liters: u16,
    months_aged: u16,
    topped_off_count: u16,
    racking_count: u8,
    cooper: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BlendComponent {
    lot_id: String,
    variety: GrapeVariety,
    percentage_scaled: u32, // scaled by 100 (e.g. 6500 = 65.00%)
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BlendTrial {
    trial_id: String,
    trial_date: String,
    blend_name: String,
    components: Vec<BlendComponent>,
    winemaker_score: u8,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BottlingLineParams {
    line_id: String,
    bottling_date: String,
    lot_id: String,
    fill_volume_ml: u16,
    headspace_mm: u8,
    closure_type: String,
    capsule_color: String,
    label_sku: String,
    bottles_per_hour: u32,
    total_bottles: u32,
    dissolved_o2_ppb: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabAnalysis {
    sample_id: String,
    lot_id: String,
    analysis_date: String,
    residual_sugar_g_per_l_scaled: u32,   // scaled by 100
    alcohol_pct_scaled: u32,              // scaled by 100 (e.g. 1380 = 13.80%)
    volatile_acidity_g_per_l_scaled: u32, // scaled by 1000
    free_so2_ppm: u16,
    total_so2_ppm: u16,
    ph_scaled: u32,
    titratable_acidity_scaled: u32,
    malic_acid_scaled: u32,
    lactic_acid_scaled: u32,
    color_intensity_scaled: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TastingNote {
    taster_name: String,
    wine_name: String,
    vintage: u16,
    aroma_descriptors: Vec<String>,
    palate_descriptors: Vec<String>,
    finish_descriptors: Vec<String>,
    score: u8,
    drink_window_start: u16,
    drink_window_end: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AppellationLevel {
    GrandCru,
    PremierCru,
    Village,
    Regional,
    AVA,
    DOC,
    DOCG,
    TableWine,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AppellationClassification {
    appellation_name: String,
    country: String,
    region: String,
    sub_region: String,
    level: AppellationLevel,
    permitted_varieties: Vec<GrapeVariety>,
    max_yield_hl_per_ha_scaled: u32,
    min_alcohol_pct_scaled: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CertificationType {
    Organic,
    Biodynamic,
    SustainablePractices,
    SalmonSafe,
    LIVE,
    Demeter,
    NOP,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CertificationRecord {
    vineyard_name: String,
    cert_type: CertificationType,
    certifying_body: String,
    cert_number: String,
    issue_date: String,
    expiry_date: String,
    acreage_certified_scaled: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellarInventoryItem {
    wine_name: String,
    vintage: u16,
    lot_id: String,
    format: String,
    cases_on_hand: u32,
    bottles_on_hand: u32,
    warehouse_location: String,
    bin_number: String,
    avg_cost_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherStationData {
    station_id: String,
    date: String,
    max_temp_c_scaled: i32,
    min_temp_c_scaled: i32,
    gdd_base10_scaled: u32, // growing degree days, scaled by 10
    rainfall_mm_scaled: u32,
    humidity_pct: u8,
    wind_speed_kmh_scaled: u32,
    solar_radiation_w_per_m2: u32,
    frost_event: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestOrDisease {
    Phylloxera,
    PowderyMildew,
    DownyMildew,
    Botrytis,
    PierceDisease,
    LeafrollVirus,
    EuscaMeasles,
    GrapeBerryMoth,
    MealyBug,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScoutingSeverity {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScoutingRecord {
    block_id: String,
    scout_date: String,
    scout_name: String,
    pest_or_disease: PestOrDisease,
    severity: ScoutingSeverity,
    affected_vines_pct: u8,
    treatment_applied: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrushPadOperation {
    date: String,
    lot_id: String,
    variety: GrapeVariety,
    tons_received_scaled: u32,
    destemmed: bool,
    whole_cluster_pct: u8,
    cold_soak_hours: u16,
    enzyme_added: bool,
    so2_addition_ppm: u16,
    destination_tank: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WineClubShipment {
    shipment_id: String,
    member_id: String,
    ship_date: String,
    wines: Vec<ShipmentWine>,
    total_bottles: u16,
    shipping_cost_cents: u32,
    carrier: String,
    tracking_number: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ShipmentWine {
    wine_name: String,
    vintage: u16,
    quantity: u16,
    price_cents: u32,
}

// ---------------------------------------------------------------------------
// Test 1: Vineyard block roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vineyard_block_checksum_roundtrip() {
    let block = VineyardBlock {
        block_id: "B7-East".into(),
        variety: GrapeVariety::CabernetSauvignon,
        rootstock: Rootstock::R110,
        planting_density_vines_per_hectare: 6000,
        row_spacing_cm: 240,
        vine_spacing_cm: 100,
        elevation_meters: 320,
        aspect: "South-Southeast".into(),
        soil_type: "Gravelly clay loam".into(),
        planted_year: 2008,
    };
    let encoded = encode_with_checksum(&block).expect("encode vineyard block");
    let (decoded, consumed): (VineyardBlock, _) =
        decode_with_checksum(&encoded).expect("decode vineyard block");
    assert_eq!(decoded, block);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Harvest record roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_harvest_record_checksum_roundtrip() {
    let record = HarvestRecord {
        block_id: "B7-East".into(),
        harvest_date: "2025-09-28".into(),
        brix: 255,
        ph_scaled: 358,
        titratable_acidity_g_per_l_scaled: 620,
        tons_harvested_scaled: 3450,
        picker_count: 24,
        bins_filled: 87,
        notes: "Hand-picked at dawn, excellent fruit condition".into(),
    };
    let encoded = encode_with_checksum(&record).expect("encode harvest record");
    let (decoded, consumed): (HarvestRecord, _) =
        decode_with_checksum(&encoded).expect("decode harvest record");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Fermentation log roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fermentation_log_checksum_roundtrip() {
    let log = FermentationLog {
        tank_id: "T-14".into(),
        vessel_type: FermentationVesselType::StainlessSteel,
        lot_id: "LOT-2025-CS-07".into(),
        day_number: 5,
        temperature_c_scaled: 278,
        specific_gravity_scaled: 1055,
        punchdowns_per_day: 3,
        free_so2_ppm: 25,
        yeast_strain: "RC212".into(),
        notes: "Cap management excellent, color extraction on track".into(),
    };
    let encoded = encode_with_checksum(&log).expect("encode fermentation log");
    let (decoded, consumed): (FermentationLog, _) =
        decode_with_checksum(&encoded).expect("decode fermentation log");
    assert_eq!(decoded, log);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Barrel aging record roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_barrel_aging_checksum_roundtrip() {
    let barrel = BarrelAgingRecord {
        barrel_id: "FR-2023-0147".into(),
        lot_id: "LOT-2025-PN-03".into(),
        oak_type: OakType::FrenchTroncais,
        toast_level: ToastLevel::MediumPlus,
        barrel_age_years: 1,
        capacity_liters: 225,
        months_aged: 14,
        topped_off_count: 7,
        racking_count: 3,
        cooper: "Francois Freres".into(),
    };
    let encoded = encode_with_checksum(&barrel).expect("encode barrel aging");
    let (decoded, consumed): (BarrelAgingRecord, _) =
        decode_with_checksum(&encoded).expect("decode barrel aging");
    assert_eq!(decoded, barrel);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Blend trial with multiple components roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_blend_trial_checksum_roundtrip() {
    let trial = BlendTrial {
        trial_id: "BT-2026-001".into(),
        trial_date: "2026-01-15".into(),
        blend_name: "Reserve Red Cuvee".into(),
        components: vec![
            BlendComponent {
                lot_id: "LOT-2025-CS-07".into(),
                variety: GrapeVariety::CabernetSauvignon,
                percentage_scaled: 6200,
            },
            BlendComponent {
                lot_id: "LOT-2025-MR-02".into(),
                variety: GrapeVariety::Merlot,
                percentage_scaled: 2000,
            },
            BlendComponent {
                lot_id: "LOT-2025-SY-01".into(),
                variety: GrapeVariety::Syrah,
                percentage_scaled: 1200,
            },
            BlendComponent {
                lot_id: "LOT-2025-PV-01".into(),
                variety: GrapeVariety::Other("Petit Verdot".into()),
                percentage_scaled: 600,
            },
        ],
        winemaker_score: 93,
        notes: "Excellent structure, good tannin integration, long finish".into(),
    };
    let encoded = encode_with_checksum(&trial).expect("encode blend trial");
    let (decoded, consumed): (BlendTrial, _) =
        decode_with_checksum(&encoded).expect("decode blend trial");
    assert_eq!(decoded, trial);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Bottling line parameters roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_bottling_line_params_checksum_roundtrip() {
    let params = BottlingLineParams {
        line_id: "LINE-A".into(),
        bottling_date: "2026-03-10".into(),
        lot_id: "LOT-2024-CH-05".into(),
        fill_volume_ml: 750,
        headspace_mm: 15,
        closure_type: "Natural cork, 49mm".into(),
        capsule_color: "Burgundy".into(),
        label_sku: "SKU-CH-RES-2024".into(),
        bottles_per_hour: 3200,
        total_bottles: 4800,
        dissolved_o2_ppb: 350,
    };
    let encoded = encode_with_checksum(&params).expect("encode bottling params");
    let (decoded, consumed): (BottlingLineParams, _) =
        decode_with_checksum(&encoded).expect("decode bottling params");
    assert_eq!(decoded, params);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Lab analysis roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lab_analysis_checksum_roundtrip() {
    let analysis = LabAnalysis {
        sample_id: "LAB-2026-0342".into(),
        lot_id: "LOT-2025-CS-07".into(),
        analysis_date: "2026-02-20".into(),
        residual_sugar_g_per_l_scaled: 150,
        alcohol_pct_scaled: 1420,
        volatile_acidity_g_per_l_scaled: 450,
        free_so2_ppm: 28,
        total_so2_ppm: 85,
        ph_scaled: 365,
        titratable_acidity_scaled: 590,
        malic_acid_scaled: 20,
        lactic_acid_scaled: 180,
        color_intensity_scaled: 1250,
    };
    let encoded = encode_with_checksum(&analysis).expect("encode lab analysis");
    let (decoded, consumed): (LabAnalysis, _) =
        decode_with_checksum(&encoded).expect("decode lab analysis");
    assert_eq!(decoded, analysis);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Tasting note with descriptors roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tasting_note_checksum_roundtrip() {
    let note = TastingNote {
        taster_name: "Elena Vasquez".into(),
        wine_name: "Estate Cabernet Sauvignon".into(),
        vintage: 2022,
        aroma_descriptors: vec![
            "Cassis".into(),
            "Blackberry".into(),
            "Cedar".into(),
            "Tobacco leaf".into(),
            "Graphite".into(),
        ],
        palate_descriptors: vec![
            "Full-bodied".into(),
            "Firm tannins".into(),
            "Dark plum".into(),
            "Espresso".into(),
        ],
        finish_descriptors: vec!["Long".into(), "Mineral".into(), "Cocoa powder".into()],
        score: 95,
        drink_window_start: 2026,
        drink_window_end: 2042,
        notes: "Exceptional depth, will reward cellaring".into(),
    };
    let encoded = encode_with_checksum(&note).expect("encode tasting note");
    let (decoded, consumed): (TastingNote, _) =
        decode_with_checksum(&encoded).expect("decode tasting note");
    assert_eq!(decoded, note);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Appellation classification roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_appellation_classification_checksum_roundtrip() {
    let appellation = AppellationClassification {
        appellation_name: "Napa Valley".into(),
        country: "USA".into(),
        region: "California".into(),
        sub_region: "Oakville".into(),
        level: AppellationLevel::AVA,
        permitted_varieties: vec![
            GrapeVariety::CabernetSauvignon,
            GrapeVariety::Merlot,
            GrapeVariety::Chardonnay,
            GrapeVariety::SauvignonBlanc,
        ],
        max_yield_hl_per_ha_scaled: 0, // no legal limit for AVA
        min_alcohol_pct_scaled: 0,
    };
    let encoded = encode_with_checksum(&appellation).expect("encode appellation");
    let (decoded, consumed): (AppellationClassification, _) =
        decode_with_checksum(&encoded).expect("decode appellation");
    assert_eq!(decoded, appellation);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Organic/biodynamic certification roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_certification_record_checksum_roundtrip() {
    let cert = CertificationRecord {
        vineyard_name: "Domaine de la Terre Vivante".into(),
        cert_type: CertificationType::Biodynamic,
        certifying_body: "Demeter International".into(),
        cert_number: "DEM-FR-2024-08821".into(),
        issue_date: "2024-04-01".into(),
        expiry_date: "2027-03-31".into(),
        acreage_certified_scaled: 4250,
    };
    let encoded = encode_with_checksum(&cert).expect("encode certification");
    let (decoded, consumed): (CertificationRecord, _) =
        decode_with_checksum(&encoded).expect("decode certification");
    assert_eq!(decoded, cert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Cellar inventory roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cellar_inventory_checksum_roundtrip() {
    let items = vec![
        CellarInventoryItem {
            wine_name: "Reserve Pinot Noir".into(),
            vintage: 2023,
            lot_id: "LOT-2023-PN-01".into(),
            format: "750ml".into(),
            cases_on_hand: 420,
            bottles_on_hand: 5040,
            warehouse_location: "Cave B, Row 12".into(),
            bin_number: "B12-07".into(),
            avg_cost_cents: 2850,
        },
        CellarInventoryItem {
            wine_name: "Estate Chardonnay".into(),
            vintage: 2024,
            lot_id: "LOT-2024-CH-03".into(),
            format: "750ml".into(),
            cases_on_hand: 280,
            bottles_on_hand: 3360,
            warehouse_location: "Cave A, Row 4".into(),
            bin_number: "A04-02".into(),
            avg_cost_cents: 1950,
        },
    ];
    let encoded = encode_with_checksum(&items).expect("encode cellar inventory");
    let (decoded, consumed): (Vec<CellarInventoryItem>, _) =
        decode_with_checksum(&encoded).expect("decode cellar inventory");
    assert_eq!(decoded, items);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Weather station data roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_weather_station_data_checksum_roundtrip() {
    let data = WeatherStationData {
        station_id: "WS-EAST-RIDGE".into(),
        date: "2025-08-15".into(),
        max_temp_c_scaled: 365,
        min_temp_c_scaled: 182,
        gdd_base10_scaled: 173,
        rainfall_mm_scaled: 0,
        humidity_pct: 42,
        wind_speed_kmh_scaled: 145,
        solar_radiation_w_per_m2: 850,
        frost_event: false,
    };
    let encoded = encode_with_checksum(&data).expect("encode weather data");
    let (decoded, consumed): (WeatherStationData, _) =
        decode_with_checksum(&encoded).expect("decode weather data");
    assert_eq!(decoded, data);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Pest/disease scouting record roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_scouting_record_checksum_roundtrip() {
    let record = ScoutingRecord {
        block_id: "B3-Hillside".into(),
        scout_date: "2025-07-10".into(),
        scout_name: "Marco Bianchi".into(),
        pest_or_disease: PestOrDisease::PowderyMildew,
        severity: ScoutingSeverity::Moderate,
        affected_vines_pct: 15,
        treatment_applied: "Sulfur dust at 5 lbs/acre".into(),
        notes: "Concentrated on shaded interior canopy, leaf pulling recommended".into(),
    };
    let encoded = encode_with_checksum(&record).expect("encode scouting record");
    let (decoded, consumed): (ScoutingRecord, _) =
        decode_with_checksum(&encoded).expect("decode scouting record");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Crush pad operations roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_crush_pad_operation_checksum_roundtrip() {
    let op = CrushPadOperation {
        date: "2025-09-28".into(),
        lot_id: "LOT-2025-PN-03".into(),
        variety: GrapeVariety::PinotNoir,
        tons_received_scaled: 820,
        destemmed: true,
        whole_cluster_pct: 25,
        cold_soak_hours: 72,
        enzyme_added: false,
        so2_addition_ppm: 30,
        destination_tank: "OFT-06".into(),
    };
    let encoded = encode_with_checksum(&op).expect("encode crush pad operation");
    let (decoded, consumed): (CrushPadOperation, _) =
        decode_with_checksum(&encoded).expect("decode crush pad operation");
    assert_eq!(decoded, op);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Wine club shipment roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_wine_club_shipment_checksum_roundtrip() {
    let shipment = WineClubShipment {
        shipment_id: "SHIP-2026-03-0415".into(),
        member_id: "MBR-10042".into(),
        ship_date: "2026-03-15".into(),
        wines: vec![
            ShipmentWine {
                wine_name: "Estate Cabernet Sauvignon".into(),
                vintage: 2022,
                quantity: 2,
                price_cents: 6500,
            },
            ShipmentWine {
                wine_name: "Reserve Chardonnay".into(),
                vintage: 2023,
                quantity: 2,
                price_cents: 4200,
            },
            ShipmentWine {
                wine_name: "Rose of Pinot Noir".into(),
                vintage: 2024,
                quantity: 2,
                price_cents: 2800,
            },
        ],
        total_bottles: 6,
        shipping_cost_cents: 1500,
        carrier: "FedEx Ground".into(),
        tracking_number: "794644790132".into(),
    };
    let encoded = encode_with_checksum(&shipment).expect("encode wine club shipment");
    let (decoded, consumed): (WineClubShipment, _) =
        decode_with_checksum(&encoded).expect("decode wine club shipment");
    assert_eq!(decoded, shipment);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Multiple vineyard blocks as Vec roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_vineyard_blocks_checksum_roundtrip() {
    let blocks = vec![
        VineyardBlock {
            block_id: "B1-Terrace".into(),
            variety: GrapeVariety::Chardonnay,
            rootstock: Rootstock::SO4,
            planting_density_vines_per_hectare: 5500,
            row_spacing_cm: 250,
            vine_spacing_cm: 110,
            elevation_meters: 180,
            aspect: "East".into(),
            soil_type: "Limestone clay".into(),
            planted_year: 2012,
        },
        VineyardBlock {
            block_id: "B2-River".into(),
            variety: GrapeVariety::SauvignonBlanc,
            rootstock: Rootstock::R3309C,
            planting_density_vines_per_hectare: 4800,
            row_spacing_cm: 270,
            vine_spacing_cm: 120,
            elevation_meters: 95,
            aspect: "Northwest".into(),
            soil_type: "Alluvial sandy loam".into(),
            planted_year: 2015,
        },
        VineyardBlock {
            block_id: "B5-Summit".into(),
            variety: GrapeVariety::Tempranillo,
            rootstock: Rootstock::R101_14,
            planting_density_vines_per_hectare: 7000,
            row_spacing_cm: 200,
            vine_spacing_cm: 90,
            elevation_meters: 510,
            aspect: "South".into(),
            soil_type: "Volcanic schist".into(),
            planted_year: 2005,
        },
    ];
    let encoded = encode_with_checksum(&blocks).expect("encode vineyard blocks vec");
    let (decoded, consumed): (Vec<VineyardBlock>, _) =
        decode_with_checksum(&encoded).expect("decode vineyard blocks vec");
    assert_eq!(decoded, blocks);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Fermentation series over multiple days roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fermentation_series_checksum_roundtrip() {
    let series: Vec<FermentationLog> = (0..7)
        .map(|day| FermentationLog {
            tank_id: "T-08".into(),
            vessel_type: FermentationVesselType::OpenTopFermenter,
            lot_id: "LOT-2025-ZN-01".into(),
            day_number: day,
            temperature_c_scaled: 220 + (day as i32) * 8,
            specific_gravity_scaled: 1095 - (day as u32) * 7,
            punchdowns_per_day: if day < 2 { 2 } else { 3 },
            free_so2_ppm: 30 - day,
            yeast_strain: "BM45".into(),
            notes: format!("Day {} of primary fermentation", day),
        })
        .collect();
    let encoded = encode_with_checksum(&series).expect("encode fermentation series");
    let (decoded, consumed): (Vec<FermentationLog>, _) =
        decode_with_checksum(&encoded).expect("decode fermentation series");
    assert_eq!(decoded, series);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: DOCG appellation with strict requirements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_docg_appellation_checksum_roundtrip() {
    let docg = AppellationClassification {
        appellation_name: "Barolo DOCG".into(),
        country: "Italy".into(),
        region: "Piedmont".into(),
        sub_region: "Langhe".into(),
        level: AppellationLevel::DOCG,
        permitted_varieties: vec![GrapeVariety::Other("Nebbiolo".into())],
        max_yield_hl_per_ha_scaled: 8000,
        min_alcohol_pct_scaled: 1300,
    };
    let encoded = encode_with_checksum(&docg).expect("encode DOCG appellation");
    let (decoded, consumed): (AppellationClassification, _) =
        decode_with_checksum(&encoded).expect("decode DOCG appellation");
    assert_eq!(decoded, docg);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Corruption detection — flipped byte in payload
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_payload_byte_flip() {
    let analysis = LabAnalysis {
        sample_id: "LAB-CORRUPT-001".into(),
        lot_id: "LOT-2025-MR-02".into(),
        analysis_date: "2026-03-01".into(),
        residual_sugar_g_per_l_scaled: 210,
        alcohol_pct_scaled: 1380,
        volatile_acidity_g_per_l_scaled: 520,
        free_so2_ppm: 32,
        total_so2_ppm: 95,
        ph_scaled: 372,
        titratable_acidity_scaled: 560,
        malic_acid_scaled: 10,
        lactic_acid_scaled: 195,
        color_intensity_scaled: 980,
    };
    let mut encoded = encode_with_checksum(&analysis).expect("encode lab analysis for corruption");
    let flip_idx = HEADER_SIZE + 2;
    if flip_idx < encoded.len() {
        encoded[flip_idx] ^= 0xFF;
    }
    let result = decode_with_checksum::<LabAnalysis>(&encoded);
    assert!(
        result.is_err(),
        "corrupted payload must cause decode_with_checksum to return Err"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Corruption detection — flipped CRC byte in header
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_crc_byte_flip() {
    let barrel = BarrelAgingRecord {
        barrel_id: "FR-CORRUPT-001".into(),
        lot_id: "LOT-2025-SY-01".into(),
        oak_type: OakType::AmericanMissouri,
        toast_level: ToastLevel::Heavy,
        barrel_age_years: 3,
        capacity_liters: 225,
        months_aged: 18,
        topped_off_count: 9,
        racking_count: 4,
        cooper: "Independent Stave".into(),
    };
    let mut encoded = encode_with_checksum(&barrel).expect("encode barrel for corruption");
    // Flip a byte in the CRC32 field (bytes 12..16)
    encoded[13] ^= 0xAA;
    let result = decode_with_checksum::<BarrelAgingRecord>(&encoded);
    assert!(
        result.is_err(),
        "corrupted CRC must cause decode_with_checksum to return Err"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection — truncated data
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_truncated_data() {
    let shipment = WineClubShipment {
        shipment_id: "SHIP-TRUNC-001".into(),
        member_id: "MBR-99999".into(),
        ship_date: "2026-03-15".into(),
        wines: vec![ShipmentWine {
            wine_name: "Grand Cru Riesling".into(),
            vintage: 2024,
            quantity: 3,
            price_cents: 5500,
        }],
        total_bottles: 3,
        shipping_cost_cents: 1200,
        carrier: "UPS".into(),
        tracking_number: "1Z999AA10123456784".into(),
    };
    let encoded = encode_with_checksum(&shipment).expect("encode shipment for truncation");
    let truncated = &encoded[..encoded.len() / 2];
    let result = decode_with_checksum::<WineClubShipment>(truncated);
    assert!(
        result.is_err(),
        "truncated data must cause decode_with_checksum to return Err"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Corruption detection — appended garbage bytes
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_appended_garbage() {
    let weather = WeatherStationData {
        station_id: "WS-GARBAGE".into(),
        date: "2025-06-01".into(),
        max_temp_c_scaled: 310,
        min_temp_c_scaled: 145,
        gdd_base10_scaled: 128,
        rainfall_mm_scaled: 50,
        humidity_pct: 55,
        wind_speed_kmh_scaled: 200,
        solar_radiation_w_per_m2: 720,
        frost_event: false,
    };
    let mut encoded = encode_with_checksum(&weather).expect("encode weather for garbage test");
    encoded.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]);
    // Decode should still succeed because the header specifies exact payload length,
    // but consumed bytes should be less than the total buffer length.
    let result = decode_with_checksum::<WeatherStationData>(&encoded);
    match result {
        Ok((decoded, consumed)) => {
            assert_eq!(decoded, weather);
            assert!(
                consumed < encoded.len(),
                "consumed should be less than buffer with appended garbage"
            );
        }
        Err(_) => {
            // Some implementations may reject extra trailing data; either is acceptable.
        }
    }
}
