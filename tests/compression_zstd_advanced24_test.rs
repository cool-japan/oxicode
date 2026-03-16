//! Advanced Zstd compression tests for OxiCode — Textile Manufacturing domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world textile manufacturing and fabric production: yarn specs,
//! weaving loom configs, knitting machine parameters, dyeing process recipes,
//! fabric quality inspections, print design metadata, finishing treatments,
//! supply chain sourcing, production batch tracking, compliance certifications,
//! inventory management, defect classification, energy/water usage tracking,
//! garment cut plan optimization, and embroidery digitization parameters.

#![cfg(feature = "compression-zstd")]
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
// Domain types — Enumerations
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FiberType {
    Cotton,
    Polyester,
    Nylon,
    Silk,
    Wool,
    Linen,
    Rayon,
    Acrylic,
    Spandex,
    Bamboo,
    Hemp,
    Cashmere,
    Tencel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TwistDirection {
    StwistClockwise,
    ZtwistCounterClockwise,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeavePattern {
    PlainTabby,
    TwillTwoOneRight,
    TwillTwoOneLeft,
    TwillThreeOneHerringbone,
    SatinFiveHarness,
    SatinEightHarness,
    BasketTwoByTwo,
    RipStop,
    Jacquard,
    Dobby,
    Leno,
    PileVelvet,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum KnitType {
    SingleJersey,
    DoubleJersey,
    Rib1x1,
    Rib2x2,
    Interlock,
    Purl,
    Jacquard,
    FrenchTerry,
    Fleece,
    Pointelle,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DyeMethod {
    ReactiveExhaust,
    ReactiveColPad,
    VatDye,
    DisperseHighTemp,
    AcidDye,
    DirectDye,
    PigmentPrint,
    IndigoDip,
    NaturalExtract,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PrintTechnique {
    RotaryScreen,
    FlatbedScreen,
    DigitalInkjet,
    BlockPrint,
    DischargeScreen,
    TransferSublimation,
    WaxResistBatik,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FinishingType {
    Mercerization,
    Sanforization,
    Calendering,
    Singeing,
    Brushing,
    Sueding,
    WaterRepellent,
    FlameRetardant,
    AntimicrobialSilver,
    Softener,
    StainRelease,
    WrinkleResistant,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CertificationType {
    OekoTexStandard100,
    OekoTexMadeInGreen,
    GotsOrganic,
    BluesignApproved,
    FairtradeCertified,
    GrsRecycled,
    CradleToCradle,
    EuEcolabel,
    UsdaOrganic,
    BetterCottonInitiative,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DefectSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DefectType {
    BrokenEnd,
    MissingPick,
    FloatDefect,
    OilStain,
    DyeSpot,
    BarreStreak,
    Slub,
    HoleSmall,
    HoleLarge,
    Crease,
    SelvageDefect,
    PatternMismatch,
    NeedleLine,
    Pilling,
    ColorShading,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StitchType {
    SatinStitch,
    RunningStitch,
    FillStitch,
    CrossStitch,
    ChainStitch,
    FrenchKnot,
    Applique,
    CutworkOpenwork,
}

// ---------------------------------------------------------------------------
// Domain types — Structs
// ---------------------------------------------------------------------------

/// Yarn specification with denier, twist, and fiber blend percentages.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YarnSpec {
    yarn_id: u64,
    lot_number: String,
    denier: u32,
    filament_count: u16,
    twist_per_meter: u16,
    twist_direction: TwistDirection,
    /// Each blend entry: (fiber type, percentage × 10, e.g. 650 = 65.0%)
    fiber_blend: Vec<(FiberType, u16)>,
    tenacity_cn_per_dtex: u32, // × 1000
    elongation_pct: u16,       // × 10
    moisture_regain_pct: u16,  // × 100
}

/// Weaving loom configuration for a single fabric construction.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeavingLoomConfig {
    loom_id: u32,
    fabric_style: String,
    warp_density_per_cm: u16, // × 10
    weft_density_per_cm: u16, // × 10
    pattern: WeavePattern,
    pattern_repeat_warp: u16,
    pattern_repeat_weft: u16,
    harness_count: u8,
    reed_count_dents_per_cm: u16,
    loom_speed_rpm: u16,
    warp_yarn_id: u64,
    weft_yarn_id: u64,
    fabric_width_mm: u16,
}

/// Knitting machine parameters for a fabric style.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KnittingMachineParams {
    machine_id: u32,
    gauge: u8, // needles per inch
    diameter_inches: u16,
    feeder_count: u8,
    knit_type: KnitType,
    stitch_length_mm: u16, // × 100
    speed_rpm: u16,
    yarn_ids: Vec<u64>,
    course_per_cm: u16, // × 10
    wales_per_cm: u16,  // × 10
    fabric_gsm_target: u16,
}

/// Dyeing process recipe for a specific color/fabric combination.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DyeingRecipe {
    recipe_id: u64,
    color_name: String,
    lab_l: u32, // CIE L*a*b* × 1000
    lab_a: i32,
    lab_b: i32,
    dye_method: DyeMethod,
    /// Each chemical: (name, grams-per-litre × 100)
    chemicals: Vec<(String, u32)>,
    temperature_c: u16, // × 10
    hold_time_minutes: u16,
    liquor_ratio: u16,        // × 10 (e.g. 80 = 1:8.0)
    wash_fastness_rating: u8, // ISO 1-5
    light_fastness_rating: u8,
    rub_fastness_dry: u8,
    rub_fastness_wet: u8,
}

/// Fabric quality inspection report.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FabricQualityInspection {
    inspection_id: u64,
    batch_id: u64,
    tensile_strength_warp_n: u32,
    tensile_strength_weft_n: u32,
    tear_strength_warp_mn: u32,
    tear_strength_weft_mn: u32,
    pilling_rating: u8,      // 1-5 (ISO 12945)
    shrinkage_warp_pct: i16, // × 100, signed (negative = growth)
    shrinkage_weft_pct: i16,
    color_fastness_washing: u8,
    abrasion_cycles: u32,      // Martindale cycles to endpoint
    air_permeability_cm3: u32, // × 100
    fabric_weight_gsm: u16,
    defect_points_per_100m: u16,
}

/// Print design metadata for a fabric print job.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrintDesignMeta {
    design_id: u64,
    design_name: String,
    technique: PrintTechnique,
    repeat_width_mm: u16,
    repeat_height_mm: u16,
    color_count: u8,
    dpi_resolution: u16,
    /// Color separations: (color_name, ink_volume_ml_per_m2 × 100)
    color_layers: Vec<(String, u32)>,
    coverage_pct: u16, // × 10
    mesh_count: u16,   // only for screen-based
}

/// Finishing treatment applied to fabric.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FinishingTreatment {
    treatment_id: u32,
    finishing_type: FinishingType,
    chemical_name: String,
    concentration_gpl: u32, // grams per litre × 100
    temperature_c: u16,
    speed_m_per_min: u16, // × 10
    curing_temp_c: u16,
    curing_time_sec: u16,
    pickup_pct: u16, // × 10
}

/// Raw material sourcing record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RawMaterialSource {
    material_id: u64,
    fiber_type: FiberType,
    origin_country: String,
    supplier_name: String,
    farm_region: String,
    harvest_year: u16,
    grade: String,
    staple_length_mm: u16, // × 10
    micronaire: u16,       // × 100 (cotton only, 0 for others)
    price_usd_per_kg: u32, // × 100
    quantity_kg: u32,
    certifications: Vec<CertificationType>,
}

/// Production batch tracking.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionBatch {
    batch_id: u64,
    fabric_style: String,
    start_timestamp: u64,
    end_timestamp: u64,
    loom_or_machine_id: u32,
    meters_produced: u32, // × 100
    meters_first_quality: u32,
    meters_second_quality: u32,
    yarn_lots_used: Vec<String>,
    operator_ids: Vec<u32>,
    efficiency_pct: u16, // × 10
    downtime_minutes: u16,
}

/// Compliance certification record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComplianceCert {
    cert_id: u64,
    certification: CertificationType,
    issued_date: String,
    expiry_date: String,
    scope_description: String,
    auditor_body: String,
    facility_name: String,
    restricted_substances_tested: Vec<String>,
    test_results_pass: Vec<bool>,
}

/// Inventory record for a fabric roll/bolt.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FabricRollInventory {
    roll_id: u64,
    batch_id: u64,
    fabric_style: String,
    color_name: String,
    width_mm: u16,
    length_m: u32,  // × 100
    weight_kg: u32, // × 1000
    quality_grade: String,
    warehouse_location: String,
    received_date: String,
    shelf_life_days: u16,
}

/// Defect classification entry from fabric inspection.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectEntry {
    defect_id: u64,
    roll_id: u64,
    position_m: u32, // metres from start × 100
    position_cm_from_selvage: u16,
    defect_type: DefectType,
    severity: DefectSeverity,
    length_cm: u16,
    width_cm: u16,
    image_reference: String,
    penalty_points: u8, // 4-point system
}

/// Energy and water usage per metre of fabric.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ResourceUsage {
    record_id: u64,
    batch_id: u64,
    electricity_kwh_per_m: u32, // × 10000
    natural_gas_mj_per_m: u32,  // × 10000
    steam_kg_per_m: u32,        // × 1000
    water_litres_per_m: u32,    // × 100
    wastewater_litres_per_m: u32,
    co2_grams_per_m: u32,
    process_stage: String,
}

/// Garment cut plan for marker-making optimization.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GarmentCutPlan {
    plan_id: u64,
    style_name: String,
    fabric_width_mm: u16,
    marker_length_mm: u32,
    marker_efficiency_pct: u16, // × 10
    /// Each piece: (piece_name, size_label, quantity, x_mm, y_mm, rotation_deg × 10)
    pieces: Vec<(String, String, u16, u32, u32, u16)>,
    layers_in_spread: u16,
    total_garments: u32,
    fabric_consumption_m: u32, // × 100
    waste_pct: u16,            // × 10
}

/// Embroidery digitization parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmbroideryDigitization {
    design_id: u64,
    design_name: String,
    stitch_count: u32,
    color_changes: u8,
    /// Each thread: (thread_brand, color_code, stitch_type, stitch_count)
    thread_layers: Vec<(String, String, StitchType, u32)>,
    design_width_mm: u16, // × 10
    design_height_mm: u16,
    density_stitches_per_cm2: u16, // × 10
    underlay_type: String,
    pull_compensation_mm: u16, // × 100
    frame_size_mm: (u16, u16),
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_yarn_spec(id: u64) -> YarnSpec {
    YarnSpec {
        yarn_id: id,
        lot_number: format!("YRN-2026-{id:05}"),
        denier: 150 + (id as u32 % 20) * 10,
        filament_count: 48 + (id as u16 % 96),
        twist_per_meter: 800 + (id as u16 % 600),
        twist_direction: if id % 2 == 0 {
            TwistDirection::StwistClockwise
        } else {
            TwistDirection::ZtwistCounterClockwise
        },
        fiber_blend: vec![(FiberType::Polyester, 650), (FiberType::Cotton, 350)],
        tenacity_cn_per_dtex: 4500 + (id as u32 % 1500),
        elongation_pct: 180 + (id as u16 % 120),
        moisture_regain_pct: 40 + (id as u16 % 30),
    }
}

fn make_loom_config(id: u32) -> WeavingLoomConfig {
    WeavingLoomConfig {
        loom_id: id,
        fabric_style: format!("STYLE-W{id:04}"),
        warp_density_per_cm: 280 + (id as u16 % 200),
        weft_density_per_cm: 220 + (id as u16 % 180),
        pattern: WeavePattern::TwillTwoOneRight,
        pattern_repeat_warp: 4 + (id as u16 % 12),
        pattern_repeat_weft: 4 + (id as u16 % 8),
        harness_count: 4 + (id as u8 % 12),
        reed_count_dents_per_cm: 120 + (id as u16 % 80),
        loom_speed_rpm: 400 + (id as u16 % 300),
        warp_yarn_id: id as u64 * 100 + 1,
        weft_yarn_id: id as u64 * 100 + 2,
        fabric_width_mm: 1500 + (id as u16 % 500),
    }
}

fn make_knitting_params(id: u32) -> KnittingMachineParams {
    KnittingMachineParams {
        machine_id: id,
        gauge: 24 + (id as u8 % 12),
        diameter_inches: 30 + (id as u16 % 10),
        feeder_count: 48 + (id as u8 % 48),
        knit_type: KnitType::SingleJersey,
        stitch_length_mm: 280 + (id as u16 % 60),
        speed_rpm: 25 + (id as u16 % 10),
        yarn_ids: (0..3).map(|i| id as u64 * 1000 + i).collect(),
        course_per_cm: 150 + (id as u16 % 50),
        wales_per_cm: 120 + (id as u16 % 40),
        fabric_gsm_target: 140 + (id as u16 % 100),
    }
}

fn make_dyeing_recipe(id: u64) -> DyeingRecipe {
    DyeingRecipe {
        recipe_id: id,
        color_name: format!("Pantone-{}", 100 + id),
        lab_l: 50000 + (id as u32 % 30000),
        lab_a: -15000 + (id as i32 % 30000),
        lab_b: -10000 + (id as i32 % 20000),
        dye_method: DyeMethod::ReactiveExhaust,
        chemicals: vec![
            (format!("Reactive Red RR-{id}"), 250 + (id as u32 % 200)),
            ("Sodium Carbonate".to_string(), 2000),
            ("Sodium Sulphate".to_string(), 5000),
            ("Sequestering Agent SA-100".to_string(), 100),
        ],
        temperature_c: 600, // 60.0 C
        hold_time_minutes: 45 + (id as u16 % 30),
        liquor_ratio: 80,
        wash_fastness_rating: 4 + (id as u8 % 2),
        light_fastness_rating: 4 + (id as u8 % 2),
        rub_fastness_dry: 4,
        rub_fastness_wet: 3 + (id as u8 % 2),
    }
}

fn make_quality_inspection(id: u64, batch: u64) -> FabricQualityInspection {
    FabricQualityInspection {
        inspection_id: id,
        batch_id: batch,
        tensile_strength_warp_n: 800 + (id as u32 % 400),
        tensile_strength_weft_n: 600 + (id as u32 % 300),
        tear_strength_warp_mn: 25000 + (id as u32 % 10000),
        tear_strength_weft_mn: 18000 + (id as u32 % 8000),
        pilling_rating: 3 + (id as u8 % 3),
        shrinkage_warp_pct: -300 + (id as i16 % 100),
        shrinkage_weft_pct: -200 + (id as i16 % 80),
        color_fastness_washing: 4,
        abrasion_cycles: 20000 + (id as u32 % 30000),
        air_permeability_cm3: 5000 + (id as u32 % 3000),
        fabric_weight_gsm: 160 + (id as u16 % 80),
        defect_points_per_100m: 10 + (id as u16 % 30),
    }
}

fn make_print_design(id: u64) -> PrintDesignMeta {
    PrintDesignMeta {
        design_id: id,
        design_name: format!("Floral-SS2026-{id:03}"),
        technique: PrintTechnique::DigitalInkjet,
        repeat_width_mm: 640 + (id as u16 % 200),
        repeat_height_mm: 914 + (id as u16 % 300),
        color_count: 6 + (id as u8 % 10),
        dpi_resolution: 600,
        color_layers: vec![
            ("Process Cyan".to_string(), 350),
            ("Process Magenta".to_string(), 280),
            ("Process Yellow".to_string(), 420),
            ("Process Black".to_string(), 180),
            (format!("Spot Red SR-{id}"), 150),
            (format!("Spot Gold SG-{id}"), 90),
        ],
        coverage_pct: 650 + (id as u16 % 200),
        mesh_count: 0, // digital, not screen
    }
}

fn make_finishing_treatment(id: u32) -> FinishingTreatment {
    FinishingTreatment {
        treatment_id: id,
        finishing_type: FinishingType::Mercerization,
        chemical_name: format!("NaOH-Tech-{id}"),
        concentration_gpl: 22000 + (id as u32 % 5000),
        temperature_c: 180 + (id as u16 % 40),
        speed_m_per_min: 250 + (id as u16 % 100),
        curing_temp_c: 0, // not applicable for mercerization
        curing_time_sec: 0,
        pickup_pct: 800 + (id as u16 % 200),
    }
}

fn make_raw_material(id: u64) -> RawMaterialSource {
    RawMaterialSource {
        material_id: id,
        fiber_type: FiberType::Cotton,
        origin_country: "Uzbekistan".to_string(),
        supplier_name: format!("GreenFiber-{id}"),
        farm_region: "Fergana Valley".to_string(),
        harvest_year: 2025,
        grade: "Middling".to_string(),
        staple_length_mm: 280 + (id as u16 % 40),
        micronaire: 420 + (id as u16 % 60),
        price_usd_per_kg: 180 + (id as u32 % 80),
        quantity_kg: 5000 + (id as u32 % 15000),
        certifications: vec![
            CertificationType::GotsOrganic,
            CertificationType::BetterCottonInitiative,
        ],
    }
}

fn make_production_batch(id: u64) -> ProductionBatch {
    ProductionBatch {
        batch_id: id,
        fabric_style: format!("STYLE-B{id:04}"),
        start_timestamp: 1_710_000_000 + id * 3600,
        end_timestamp: 1_710_000_000 + id * 3600 + 28800,
        loom_or_machine_id: (id % 50) as u32,
        meters_produced: 50000 + (id as u32 % 10000),
        meters_first_quality: 48000 + (id as u32 % 9000),
        meters_second_quality: 1500 + (id as u32 % 1000),
        yarn_lots_used: (0..3)
            .map(|i| format!("YRN-2026-{:05}", id * 10 + i))
            .collect(),
        operator_ids: vec![1001, 1002, 1003],
        efficiency_pct: 850 + (id as u16 % 100),
        downtime_minutes: 15 + (id as u16 % 60),
    }
}

fn make_compliance_cert(id: u64) -> ComplianceCert {
    ComplianceCert {
        cert_id: id,
        certification: CertificationType::OekoTexStandard100,
        issued_date: format!("2026-01-{:02}", 1 + id % 28),
        expiry_date: format!("2027-01-{:02}", 1 + id % 28),
        scope_description: format!("Baby textiles Class I, Facility F-{id}"),
        auditor_body: "Hohenstein Institute".to_string(),
        facility_name: format!("Mill-{id:03}"),
        restricted_substances_tested: vec![
            "Formaldehyde".to_string(),
            "Extractable Heavy Metals".to_string(),
            "Phthalates".to_string(),
            "Pesticides".to_string(),
            "Chlorinated Phenols".to_string(),
            "Organotin Compounds".to_string(),
        ],
        test_results_pass: vec![true, true, true, true, true, true],
    }
}

fn make_roll_inventory(id: u64, batch: u64) -> FabricRollInventory {
    FabricRollInventory {
        roll_id: id,
        batch_id: batch,
        fabric_style: format!("STYLE-R{batch:04}"),
        color_name: "Navy Blue NB-042".to_string(),
        width_mm: 1500,
        length_m: 8000 + (id as u32 % 4000),
        weight_kg: 24000 + (id as u32 % 12000),
        quality_grade: "A".to_string(),
        warehouse_location: format!("WH-3/R{}/{}", (id % 20) + 1, (id % 50) + 1),
        received_date: "2026-03-10".to_string(),
        shelf_life_days: 365,
    }
}

fn make_defect_entry(id: u64, roll: u64) -> DefectEntry {
    DefectEntry {
        defect_id: id,
        roll_id: roll,
        position_m: 1200 + (id as u32 % 6000),
        position_cm_from_selvage: 20 + (id as u16 % 120),
        defect_type: DefectType::BrokenEnd,
        severity: DefectSeverity::Minor,
        length_cm: 2 + (id as u16 % 8),
        width_cm: 1,
        image_reference: format!("IMG-DEF-{id:08}.jpg"),
        penalty_points: 1,
    }
}

fn make_resource_usage(id: u64, batch: u64) -> ResourceUsage {
    ResourceUsage {
        record_id: id,
        batch_id: batch,
        electricity_kwh_per_m: 3500 + (id as u32 % 2000),
        natural_gas_mj_per_m: 8000 + (id as u32 % 4000),
        steam_kg_per_m: 1200 + (id as u32 % 800),
        water_litres_per_m: 15000 + (id as u32 % 10000),
        wastewater_litres_per_m: 12000 + (id as u32 % 8000),
        co2_grams_per_m: 450 + (id as u32 % 200),
        process_stage: "Dyeing-Finishing".to_string(),
    }
}

fn make_cut_plan(id: u64) -> GarmentCutPlan {
    GarmentCutPlan {
        plan_id: id,
        style_name: format!("T-Shirt-Basic-{id:03}"),
        fabric_width_mm: 1500,
        marker_length_mm: 12000 + (id as u32 % 5000),
        marker_efficiency_pct: 850 + (id as u16 % 100),
        pieces: vec![
            ("Front Body".to_string(), "M".to_string(), 2, 0, 0, 0),
            ("Back Body".to_string(), "M".to_string(), 2, 500, 0, 0),
            ("Sleeve Left".to_string(), "M".to_string(), 2, 0, 600, 900),
            (
                "Sleeve Right".to_string(),
                "M".to_string(),
                2,
                350,
                600,
                2700,
            ),
            ("Collar".to_string(), "M".to_string(), 2, 700, 600, 0),
            ("Front Body".to_string(), "L".to_string(), 2, 0, 1200, 0),
            ("Back Body".to_string(), "L".to_string(), 2, 520, 1200, 0),
        ],
        layers_in_spread: 60 + (id as u16 % 40),
        total_garments: 120 + (id as u32 % 80),
        fabric_consumption_m: 14500 + (id as u32 % 3000),
        waste_pct: 120 + (id as u16 % 50),
    }
}

fn make_embroidery(id: u64) -> EmbroideryDigitization {
    EmbroideryDigitization {
        design_id: id,
        design_name: format!("Logo-Corp-{id:04}"),
        stitch_count: 8000 + (id as u32 % 20000),
        color_changes: 4 + (id as u8 % 8),
        thread_layers: vec![
            (
                "Madeira".to_string(),
                "1147".to_string(),
                StitchType::SatinStitch,
                3000 + (id as u32 % 2000),
            ),
            (
                "Madeira".to_string(),
                "1000".to_string(),
                StitchType::FillStitch,
                4000 + (id as u32 % 3000),
            ),
            (
                "Madeira".to_string(),
                "1312".to_string(),
                StitchType::RunningStitch,
                1000 + (id as u32 % 800),
            ),
        ],
        design_width_mm: 800 + (id as u16 % 400),
        design_height_mm: 600 + (id as u16 % 300),
        density_stitches_per_cm2: 400 + (id as u16 % 100),
        underlay_type: "Center Run + Edge Walk".to_string(),
        pull_compensation_mm: 30 + (id as u16 % 20),
        frame_size_mm: (240, 240),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_yarn_specifications_roundtrip() {
    let yarns: Vec<YarnSpec> = (1..=30).map(make_yarn_spec).collect();
    let encoded = encode_to_vec(&yarns).expect("encode yarn specs");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress yarn specs");
    let decompressed = decompress(&compressed).expect("decompress yarn specs");
    let (decoded, _): (Vec<YarnSpec>, _) =
        decode_from_slice(&decompressed).expect("decode yarn specs");
    assert_eq!(yarns, decoded);
}

#[test]
fn test_weaving_loom_configs_roundtrip() {
    let configs: Vec<WeavingLoomConfig> = (1..=25).map(make_loom_config).collect();
    let encoded = encode_to_vec(&configs).expect("encode loom configs");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress loom configs");
    let decompressed = decompress(&compressed).expect("decompress loom configs");
    let (decoded, _): (Vec<WeavingLoomConfig>, _) =
        decode_from_slice(&decompressed).expect("decode loom configs");
    assert_eq!(configs, decoded);
}

#[test]
fn test_knitting_machine_params_roundtrip() {
    let params: Vec<KnittingMachineParams> = (1..=20).map(make_knitting_params).collect();
    let encoded = encode_to_vec(&params).expect("encode knitting params");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress knitting params");
    let decompressed = decompress(&compressed).expect("decompress knitting params");
    let (decoded, _): (Vec<KnittingMachineParams>, _) =
        decode_from_slice(&decompressed).expect("decode knitting params");
    assert_eq!(params, decoded);
}

#[test]
fn test_dyeing_recipes_roundtrip() {
    let recipes: Vec<DyeingRecipe> = (1..=15).map(make_dyeing_recipe).collect();
    let encoded = encode_to_vec(&recipes).expect("encode dyeing recipes");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress dyeing recipes");
    let decompressed = decompress(&compressed).expect("decompress dyeing recipes");
    let (decoded, _): (Vec<DyeingRecipe>, _) =
        decode_from_slice(&decompressed).expect("decode dyeing recipes");
    assert_eq!(recipes, decoded);
}

#[test]
fn test_fabric_quality_inspections_roundtrip() {
    let inspections: Vec<FabricQualityInspection> = (1..=20)
        .map(|i| make_quality_inspection(i, i * 100))
        .collect();
    let encoded = encode_to_vec(&inspections).expect("encode inspections");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress inspections");
    let decompressed = decompress(&compressed).expect("decompress inspections");
    let (decoded, _): (Vec<FabricQualityInspection>, _) =
        decode_from_slice(&decompressed).expect("decode inspections");
    assert_eq!(inspections, decoded);
}

#[test]
fn test_print_designs_roundtrip() {
    let designs: Vec<PrintDesignMeta> = (1..=18).map(make_print_design).collect();
    let encoded = encode_to_vec(&designs).expect("encode print designs");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress print designs");
    let decompressed = decompress(&compressed).expect("decompress print designs");
    let (decoded, _): (Vec<PrintDesignMeta>, _) =
        decode_from_slice(&decompressed).expect("decode print designs");
    assert_eq!(designs, decoded);
}

#[test]
fn test_finishing_treatments_roundtrip() {
    let treatments: Vec<FinishingTreatment> = (1..=12).map(make_finishing_treatment).collect();
    let encoded = encode_to_vec(&treatments).expect("encode finishing treatments");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress finishing treatments");
    let decompressed = decompress(&compressed).expect("decompress finishing treatments");
    let (decoded, _): (Vec<FinishingTreatment>, _) =
        decode_from_slice(&decompressed).expect("decode finishing treatments");
    assert_eq!(treatments, decoded);
}

#[test]
fn test_raw_material_sourcing_roundtrip() {
    let materials: Vec<RawMaterialSource> = (1..=10).map(make_raw_material).collect();
    let encoded = encode_to_vec(&materials).expect("encode raw materials");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress raw materials");
    let decompressed = decompress(&compressed).expect("decompress raw materials");
    let (decoded, _): (Vec<RawMaterialSource>, _) =
        decode_from_slice(&decompressed).expect("decode raw materials");
    assert_eq!(materials, decoded);
}

#[test]
fn test_production_batch_tracking_roundtrip() {
    let batches: Vec<ProductionBatch> = (1..=25).map(make_production_batch).collect();
    let encoded = encode_to_vec(&batches).expect("encode batches");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress batches");
    let decompressed = decompress(&compressed).expect("decompress batches");
    let (decoded, _): (Vec<ProductionBatch>, _) =
        decode_from_slice(&decompressed).expect("decode batches");
    assert_eq!(batches, decoded);
}

#[test]
fn test_compliance_certifications_roundtrip() {
    let certs: Vec<ComplianceCert> = (1..=8).map(make_compliance_cert).collect();
    let encoded = encode_to_vec(&certs).expect("encode compliance certs");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress compliance certs");
    let decompressed = decompress(&compressed).expect("decompress compliance certs");
    let (decoded, _): (Vec<ComplianceCert>, _) =
        decode_from_slice(&decompressed).expect("decode compliance certs");
    assert_eq!(certs, decoded);
}

#[test]
fn test_fabric_roll_inventory_roundtrip() {
    let rolls: Vec<FabricRollInventory> = (1..=40)
        .map(|i| make_roll_inventory(i, i / 4 + 1))
        .collect();
    let encoded = encode_to_vec(&rolls).expect("encode rolls");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress rolls");
    let decompressed = decompress(&compressed).expect("decompress rolls");
    let (decoded, _): (Vec<FabricRollInventory>, _) =
        decode_from_slice(&decompressed).expect("decode rolls");
    assert_eq!(rolls, decoded);
}

#[test]
fn test_defect_classification_roundtrip() {
    let defects: Vec<DefectEntry> = (1..=50).map(|i| make_defect_entry(i, i / 5 + 1)).collect();
    let encoded = encode_to_vec(&defects).expect("encode defects");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress defects");
    let decompressed = decompress(&compressed).expect("decompress defects");
    let (decoded, _): (Vec<DefectEntry>, _) =
        decode_from_slice(&decompressed).expect("decode defects");
    assert_eq!(defects, decoded);
}

#[test]
fn test_energy_water_usage_roundtrip() {
    let records: Vec<ResourceUsage> = (1..=30).map(|i| make_resource_usage(i, i * 10)).collect();
    let encoded = encode_to_vec(&records).expect("encode resource usage");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress resource usage");
    let decompressed = decompress(&compressed).expect("decompress resource usage");
    let (decoded, _): (Vec<ResourceUsage>, _) =
        decode_from_slice(&decompressed).expect("decode resource usage");
    assert_eq!(records, decoded);
}

#[test]
fn test_garment_cut_plan_roundtrip() {
    let plans: Vec<GarmentCutPlan> = (1..=10).map(make_cut_plan).collect();
    let encoded = encode_to_vec(&plans).expect("encode cut plans");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress cut plans");
    let decompressed = decompress(&compressed).expect("decompress cut plans");
    let (decoded, _): (Vec<GarmentCutPlan>, _) =
        decode_from_slice(&decompressed).expect("decode cut plans");
    assert_eq!(plans, decoded);
}

#[test]
fn test_embroidery_digitization_roundtrip() {
    let designs: Vec<EmbroideryDigitization> = (1..=12).map(make_embroidery).collect();
    let encoded = encode_to_vec(&designs).expect("encode embroidery");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress embroidery");
    let decompressed = decompress(&compressed).expect("decompress embroidery");
    let (decoded, _): (Vec<EmbroideryDigitization>, _) =
        decode_from_slice(&decompressed).expect("decode embroidery");
    assert_eq!(designs, decoded);
}

#[test]
fn test_yarn_specs_compression_ratio() {
    let yarns: Vec<YarnSpec> = (1..=200).map(make_yarn_spec).collect();
    let encoded = encode_to_vec(&yarns).expect("encode yarns for ratio");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress yarns for ratio");
    assert!(
        compressed.len() < encoded.len(),
        "zstd should reduce yarn spec data: compressed {} >= raw {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_production_batches_compression_ratio() {
    let batches: Vec<ProductionBatch> = (1..=100).map(make_production_batch).collect();
    let encoded = encode_to_vec(&batches).expect("encode batches for ratio");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress batches for ratio");
    assert!(
        compressed.len() < encoded.len(),
        "zstd should reduce batch data: compressed {} >= raw {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_full_supply_chain_composite_roundtrip() {
    // Build a composite supply-chain snapshot: materials + yarns + batches + rolls.
    let materials: Vec<RawMaterialSource> = (1..=5).map(make_raw_material).collect();
    let yarns: Vec<YarnSpec> = (1..=10).map(make_yarn_spec).collect();
    let batches: Vec<ProductionBatch> = (1..=5).map(make_production_batch).collect();
    let rolls: Vec<FabricRollInventory> = (1..=20)
        .map(|i| make_roll_inventory(i, i / 4 + 1))
        .collect();

    let composite = (
        materials.clone(),
        yarns.clone(),
        batches.clone(),
        rolls.clone(),
    );
    let encoded = encode_to_vec(&composite).expect("encode composite");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress composite");
    let decompressed = decompress(&compressed).expect("decompress composite");
    let (decoded, _): (
        (
            Vec<RawMaterialSource>,
            Vec<YarnSpec>,
            Vec<ProductionBatch>,
            Vec<FabricRollInventory>,
        ),
        _,
    ) = decode_from_slice(&decompressed).expect("decode composite");
    assert_eq!(decoded.0, materials);
    assert_eq!(decoded.1, yarns);
    assert_eq!(decoded.2, batches);
    assert_eq!(decoded.3, rolls);
}

#[test]
fn test_quality_defect_pipeline_roundtrip() {
    // Quality pipeline: inspections → defect entries → resource usage.
    let inspections: Vec<FabricQualityInspection> = (1..=5)
        .map(|i| make_quality_inspection(i, i * 100))
        .collect();
    let defects: Vec<DefectEntry> = (1..=15).map(|i| make_defect_entry(i, i / 3 + 1)).collect();
    let resources: Vec<ResourceUsage> = (1..=5).map(|i| make_resource_usage(i, i * 100)).collect();

    let pipeline = (inspections.clone(), defects.clone(), resources.clone());
    let encoded = encode_to_vec(&pipeline).expect("encode pipeline");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress pipeline");
    let decompressed = decompress(&compressed).expect("decompress pipeline");
    let (decoded, _): (
        (
            Vec<FabricQualityInspection>,
            Vec<DefectEntry>,
            Vec<ResourceUsage>,
        ),
        _,
    ) = decode_from_slice(&decompressed).expect("decode pipeline");
    assert_eq!(decoded.0, inspections);
    assert_eq!(decoded.1, defects);
    assert_eq!(decoded.2, resources);
}

#[test]
fn test_design_to_production_workflow_roundtrip() {
    // Design workflow: print design + cut plan + embroidery.
    let prints: Vec<PrintDesignMeta> = (1..=4).map(make_print_design).collect();
    let plans: Vec<GarmentCutPlan> = (1..=4).map(make_cut_plan).collect();
    let embroideries: Vec<EmbroideryDigitization> = (1..=4).map(make_embroidery).collect();

    let workflow = (prints.clone(), plans.clone(), embroideries.clone());
    let encoded = encode_to_vec(&workflow).expect("encode workflow");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress workflow");
    let decompressed = decompress(&compressed).expect("decompress workflow");
    let (decoded, _): (
        (
            Vec<PrintDesignMeta>,
            Vec<GarmentCutPlan>,
            Vec<EmbroideryDigitization>,
        ),
        _,
    ) = decode_from_slice(&decompressed).expect("decode workflow");
    assert_eq!(decoded.0, prints);
    assert_eq!(decoded.1, plans);
    assert_eq!(decoded.2, embroideries);
}

#[test]
fn test_compliance_and_certification_compression_ratio() {
    // Certification records contain repetitive strings; zstd should compress well.
    let certs: Vec<ComplianceCert> = (1..=50).map(make_compliance_cert).collect();
    let encoded = encode_to_vec(&certs).expect("encode certs for ratio");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress certs for ratio");
    assert!(
        compressed.len() < encoded.len(),
        "zstd should reduce cert data: compressed {} >= raw {}",
        compressed.len(),
        encoded.len()
    );
    // Verify round-trip still works.
    let decompressed = decompress(&compressed).expect("decompress certs for ratio");
    let (decoded, _): (Vec<ComplianceCert>, _) =
        decode_from_slice(&decompressed).expect("decode certs for ratio");
    assert_eq!(certs, decoded);
}

#[test]
fn test_large_defect_log_compression_ratio() {
    let defects: Vec<DefectEntry> = (1..=500)
        .map(|i| make_defect_entry(i, i / 10 + 1))
        .collect();
    let encoded = encode_to_vec(&defects).expect("encode large defect log");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress large defect log");
    assert!(
        compressed.len() < encoded.len(),
        "zstd should reduce defect log: compressed {} >= raw {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large defect log");
    let (decoded, _): (Vec<DefectEntry>, _) =
        decode_from_slice(&decompressed).expect("decode large defect log");
    assert_eq!(defects, decoded);
}
