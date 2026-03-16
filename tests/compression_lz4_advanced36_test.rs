//! Advanced LZ4 compression tests for the textile manufacturing and fabric production domain.
//!
//! Covers yarn specifications (denier, twist, ply), loom settings (warp/weft density,
//! pattern repeats), dyeing bath formulations, fabric tensile strength tests,
//! weaving defect classifications, spinning frame parameters, finishing treatment
//! sequences, color fastness ratings, fiber blend compositions, quality grade
//! assignments, and related textile engineering data.

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
// Helper
// ---------------------------------------------------------------------------

fn lz4_roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T, label: &str) {
    let enc = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {label}"));
    let compressed =
        compress(&enc, Compression::Lz4).unwrap_or_else(|_| panic!("compress {label}"));
    let decompressed = decompress(&compressed).unwrap_or_else(|_| panic!("decompress {label}"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&decompressed).unwrap_or_else(|_| panic!("decode {label}"));
    assert_eq!(*val, decoded, "{label}: roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
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
    Custom(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct YarnSpecification {
    yarn_id: u64,
    fiber: FiberType,
    denier: f64,
    tex_count: f64,
    twist_per_meter: u32,
    twist_direction: String,
    ply_count: u8,
    tenacity_cn_per_dtex: f64,
    elongation_pct: f64,
    moisture_regain_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LoomSettings {
    loom_id: String,
    warp_density_per_cm: f64,
    weft_density_per_cm: f64,
    pattern_repeat_warp: u32,
    pattern_repeat_weft: u32,
    reed_count: u32,
    harness_count: u8,
    picks_per_minute: u32,
    fabric_width_cm: f64,
    tension_newtons: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DyeClass {
    Reactive,
    Disperse,
    Acid,
    Basic,
    Vat,
    Sulfur,
    Direct,
    Mordant,
    Pigment,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DyeingBathFormulation {
    bath_id: u64,
    dye_class: DyeClass,
    dye_name: String,
    concentration_g_per_l: f64,
    temperature_celsius: f64,
    ph_level: f64,
    duration_minutes: u32,
    salt_g_per_l: f64,
    alkali_g_per_l: f64,
    liquor_ratio: String,
    auxiliaries: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TensileStrengthTest {
    specimen_id: String,
    gauge_length_mm: f64,
    width_mm: f64,
    thickness_mm: f64,
    breaking_force_n: f64,
    breaking_elongation_pct: f64,
    tensile_strength_mpa: f64,
    modulus_mpa: f64,
    test_speed_mm_per_min: f64,
    warp_direction: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WeavingDefect {
    BrokenEnd { warp_position: u32 },
    BrokenPick { pick_number: u64 },
    FloatLong { length_mm: f64, direction: String },
    MissingEnd { count: u32 },
    ReedMark { severity: u8 },
    TempleCrease { distance_from_edge_cm: f64 },
    Slub { diameter_mm: f64 },
    OilStain { area_sq_cm: f64 },
    WeftBar { width_cm: f64 },
    SelvedgeFault { side: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefectReport {
    report_id: u64,
    fabric_roll_id: String,
    inspector_code: String,
    defects: Vec<WeavingDefect>,
    total_length_inspected_m: f64,
    defect_points_per_100m: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SpinningFrameParams {
    frame_id: String,
    spindle_speed_rpm: u32,
    draft_ratio: f64,
    twist_multiplier: f64,
    ring_diameter_mm: f64,
    traveller_number: u16,
    bobbin_length_mm: f64,
    break_rate_per_1000_spindle_hrs: f64,
    roving_hank: f64,
    yarn_count_ne: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FinishingTreatment {
    Singeing {
        flame_intensity: u8,
    },
    Desizing {
        enzyme_concentration_pct: f64,
    },
    Scouring {
        caustic_g_per_l: f64,
        temperature_c: f64,
    },
    Bleaching {
        h2o2_g_per_l: f64,
        duration_min: u32,
    },
    Mercerizing {
        naoh_baume: f64,
        tension_applied: bool,
    },
    Calendering {
        pressure_kn: f64,
        temperature_c: f64,
        nip_count: u8,
    },
    Sanforizing {
        shrinkage_target_pct: f64,
    },
    SoftenerApplication {
        product_name: String,
        dosage_g_per_l: f64,
    },
    WaterRepellent {
        fluorocarbon_free: bool,
        rating: u8,
    },
    HeatSetting {
        temperature_c: f64,
        duration_seconds: u32,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FinishingSequence {
    sequence_id: u64,
    fabric_type: String,
    treatments: Vec<FinishingTreatment>,
    final_width_cm: f64,
    final_gsm: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ColorFastnessTest {
    WashFastness {
        iso_rating: u8,
        cycles: u32,
    },
    LightFastness {
        blue_wool_scale: u8,
        hours_exposed: u32,
    },
    RubFastness {
        dry_rating: u8,
        wet_rating: u8,
    },
    PerspFastness {
        acidic_rating: u8,
        alkaline_rating: u8,
    },
    WaterFastness {
        rating: u8,
    },
    ChlorineFastness {
        rating: u8,
        ppm: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColorFastnessReport {
    report_id: u64,
    dye_lot: String,
    color_name: String,
    substrate: String,
    tests: Vec<ColorFastnessTest>,
    overall_pass: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FiberBlendComponent {
    fiber: FiberType,
    percentage: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FiberBlendComposition {
    blend_id: String,
    components: Vec<FiberBlendComponent>,
    intended_use: String,
    trade_name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum QualityGrade {
    Premium,
    First,
    Second,
    Third,
    Rejected { reason: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FabricQualityAssignment {
    roll_id: String,
    grade: QualityGrade,
    gsm: f64,
    width_cm: f64,
    length_m: f64,
    defect_points: u32,
    color_delta_e: f64,
    shrinkage_warp_pct: f64,
    shrinkage_weft_pct: f64,
    pilling_rating: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WarpBeamPreparation {
    beam_id: String,
    total_ends: u32,
    yarn_count: f64,
    beam_width_cm: f64,
    warp_length_m: f64,
    sizing_recipe: String,
    sizing_pickup_pct: f64,
    creel_capacity: u32,
    tension_grams_per_end: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WeavePattern {
    Plain,
    Twill { direction: String, repeat: u8 },
    Satin { harness: u8, step: u8 },
    Basket { warp_group: u8, weft_group: u8 },
    Dobby { pattern_name: String, shafts: u8 },
    Jacquard { design_file: String, hooks: u32 },
    RibWarp { rib_width: u8 },
    RibWeft { rib_width: u8 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FabricDesignSpec {
    design_id: u64,
    pattern: WeavePattern,
    warp_yarn: YarnSpecification,
    weft_yarn: YarnSpecification,
    epi: u32,
    ppi: u32,
    target_gsm: f64,
    target_width_cm: f64,
    crimp_warp_pct: f64,
    crimp_weft_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KnittingMachineParams {
    machine_id: String,
    gauge: u8,
    diameter_inches: f64,
    feeder_count: u16,
    needle_count: u32,
    speed_rpm: u32,
    stitch_length_mm: f64,
    yarn_tension_cn: f64,
    fabric_takedown_speed: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TextileTestBatch {
    batch_id: String,
    fabric_type: String,
    air_permeability_l_per_m2_s: f64,
    water_vapor_resistance: f64,
    thermal_resistance_clo: f64,
    abrasion_cycles_to_failure: u32,
    tear_strength_warp_n: f64,
    tear_strength_weft_n: f64,
    bursting_strength_kpa: f64,
    drape_coefficient: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PrintingPass {
    pass_number: u8,
    color_channel: String,
    ink_volume_ml_per_m2: f64,
    resolution_dpi: u32,
    curing_temp_c: f64,
    curing_time_s: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DigitalPrintJob {
    job_id: u64,
    fabric_substrate: String,
    pretreatment: String,
    passes: Vec<PrintingPass>,
    total_area_m2: f64,
    color_gamut_coverage_pct: f64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_yarn_specification_lz4() {
    let val = YarnSpecification {
        yarn_id: 100_001,
        fiber: FiberType::Cotton,
        denier: 150.0,
        tex_count: 16.67,
        twist_per_meter: 820,
        twist_direction: String::from("Z"),
        ply_count: 2,
        tenacity_cn_per_dtex: 3.5,
        elongation_pct: 7.2,
        moisture_regain_pct: 8.5,
    };
    lz4_roundtrip(&val, "yarn specification");
}

#[test]
fn test_loom_settings_lz4() {
    let val = LoomSettings {
        loom_id: String::from("LOOM-A17"),
        warp_density_per_cm: 42.0,
        weft_density_per_cm: 36.0,
        pattern_repeat_warp: 8,
        pattern_repeat_weft: 8,
        reed_count: 84,
        harness_count: 8,
        picks_per_minute: 450,
        fabric_width_cm: 160.0,
        tension_newtons: 2200.0,
    };
    lz4_roundtrip(&val, "loom settings");
}

#[test]
fn test_dyeing_bath_formulation_lz4() {
    let val = DyeingBathFormulation {
        bath_id: 55_201,
        dye_class: DyeClass::Reactive,
        dye_name: String::from("Cibacron Blue FN-R"),
        concentration_g_per_l: 30.0,
        temperature_celsius: 60.0,
        ph_level: 11.0,
        duration_minutes: 45,
        salt_g_per_l: 50.0,
        alkali_g_per_l: 15.0,
        liquor_ratio: String::from("1:10"),
        auxiliaries: vec![
            String::from("Leveling agent 2 g/L"),
            String::from("Sequestering agent 1 g/L"),
            String::from("Antifoam 0.5 g/L"),
        ],
    };
    lz4_roundtrip(&val, "dyeing bath formulation");
}

#[test]
fn test_tensile_strength_warp_direction_lz4() {
    let val = TensileStrengthTest {
        specimen_id: String::from("TST-W-0042"),
        gauge_length_mm: 200.0,
        width_mm: 50.0,
        thickness_mm: 0.38,
        breaking_force_n: 685.0,
        breaking_elongation_pct: 12.5,
        tensile_strength_mpa: 36.1,
        modulus_mpa: 420.0,
        test_speed_mm_per_min: 100.0,
        warp_direction: true,
    };
    lz4_roundtrip(&val, "tensile strength warp");
}

#[test]
fn test_tensile_strength_weft_direction_lz4() {
    let val = TensileStrengthTest {
        specimen_id: String::from("TST-F-0043"),
        gauge_length_mm: 200.0,
        width_mm: 50.0,
        thickness_mm: 0.38,
        breaking_force_n: 420.0,
        breaking_elongation_pct: 18.3,
        tensile_strength_mpa: 22.1,
        modulus_mpa: 280.0,
        test_speed_mm_per_min: 100.0,
        warp_direction: false,
    };
    lz4_roundtrip(&val, "tensile strength weft");
}

#[test]
fn test_weaving_defect_report_lz4() {
    let val = DefectReport {
        report_id: 8_801,
        fabric_roll_id: String::from("ROLL-2026-03-0154"),
        inspector_code: String::from("INS-07"),
        defects: vec![
            WeavingDefect::BrokenEnd { warp_position: 342 },
            WeavingDefect::FloatLong {
                length_mm: 8.5,
                direction: String::from("weft"),
            },
            WeavingDefect::OilStain { area_sq_cm: 1.2 },
            WeavingDefect::ReedMark { severity: 3 },
            WeavingDefect::Slub { diameter_mm: 2.1 },
        ],
        total_length_inspected_m: 120.0,
        defect_points_per_100m: 18.3,
    };
    lz4_roundtrip(&val, "defect report");
}

#[test]
fn test_spinning_frame_parameters_lz4() {
    let val = SpinningFrameParams {
        frame_id: String::from("RF-320"),
        spindle_speed_rpm: 18_000,
        draft_ratio: 35.0,
        twist_multiplier: 3.8,
        ring_diameter_mm: 42.0,
        traveller_number: 5,
        bobbin_length_mm: 180.0,
        break_rate_per_1000_spindle_hrs: 12.5,
        roving_hank: 0.85,
        yarn_count_ne: 30.0,
    };
    lz4_roundtrip(&val, "spinning frame params");
}

#[test]
fn test_finishing_sequence_lz4() {
    let val = FinishingSequence {
        sequence_id: 7_700,
        fabric_type: String::from("100% Cotton Poplin"),
        treatments: vec![
            FinishingTreatment::Singeing { flame_intensity: 7 },
            FinishingTreatment::Desizing {
                enzyme_concentration_pct: 2.5,
            },
            FinishingTreatment::Scouring {
                caustic_g_per_l: 25.0,
                temperature_c: 98.0,
            },
            FinishingTreatment::Bleaching {
                h2o2_g_per_l: 35.0,
                duration_min: 60,
            },
            FinishingTreatment::Mercerizing {
                naoh_baume: 28.0,
                tension_applied: true,
            },
            FinishingTreatment::SoftenerApplication {
                product_name: String::from("Silicone Macro Emulsion"),
                dosage_g_per_l: 20.0,
            },
            FinishingTreatment::Sanforizing {
                shrinkage_target_pct: 1.0,
            },
        ],
        final_width_cm: 150.0,
        final_gsm: 135.0,
    };
    lz4_roundtrip(&val, "finishing sequence");
}

#[test]
fn test_color_fastness_report_lz4() {
    let val = ColorFastnessReport {
        report_id: 3_310,
        dye_lot: String::from("DL-2026-0987"),
        color_name: String::from("Indigo Navy"),
        substrate: String::from("Cotton Twill 3/1"),
        tests: vec![
            ColorFastnessTest::WashFastness {
                iso_rating: 4,
                cycles: 25,
            },
            ColorFastnessTest::LightFastness {
                blue_wool_scale: 6,
                hours_exposed: 200,
            },
            ColorFastnessTest::RubFastness {
                dry_rating: 4,
                wet_rating: 3,
            },
            ColorFastnessTest::PerspFastness {
                acidic_rating: 4,
                alkaline_rating: 4,
            },
            ColorFastnessTest::WaterFastness { rating: 4 },
            ColorFastnessTest::ChlorineFastness {
                rating: 3,
                ppm: 20.0,
            },
        ],
        overall_pass: true,
    };
    lz4_roundtrip(&val, "color fastness report");
}

#[test]
fn test_fiber_blend_composition_lz4() {
    let val = FiberBlendComposition {
        blend_id: String::from("BLD-PC6535"),
        components: vec![
            FiberBlendComponent {
                fiber: FiberType::Polyester,
                percentage: 65.0,
            },
            FiberBlendComponent {
                fiber: FiberType::Cotton,
                percentage: 35.0,
            },
        ],
        intended_use: String::from("Shirting"),
        trade_name: String::from("PolyCot Classic"),
    };
    lz4_roundtrip(&val, "fiber blend composition");
}

#[test]
fn test_quality_grade_premium_lz4() {
    let val = FabricQualityAssignment {
        roll_id: String::from("R-20260315-001"),
        grade: QualityGrade::Premium,
        gsm: 142.0,
        width_cm: 152.0,
        length_m: 85.0,
        defect_points: 2,
        color_delta_e: 0.3,
        shrinkage_warp_pct: 1.2,
        shrinkage_weft_pct: 0.8,
        pilling_rating: 5,
    };
    lz4_roundtrip(&val, "quality grade premium");
}

#[test]
fn test_quality_grade_rejected_lz4() {
    let val = FabricQualityAssignment {
        roll_id: String::from("R-20260315-099"),
        grade: QualityGrade::Rejected {
            reason: String::from("Excessive barre defects in weft direction"),
        },
        gsm: 138.0,
        width_cm: 149.5,
        length_m: 42.0,
        defect_points: 87,
        color_delta_e: 2.8,
        shrinkage_warp_pct: 4.1,
        shrinkage_weft_pct: 3.6,
        pilling_rating: 2,
    };
    lz4_roundtrip(&val, "quality grade rejected");
}

#[test]
fn test_warp_beam_preparation_lz4() {
    let val = WarpBeamPreparation {
        beam_id: String::from("WB-6420"),
        total_ends: 6720,
        yarn_count: 40.0,
        beam_width_cm: 170.0,
        warp_length_m: 3200.0,
        sizing_recipe: String::from("PVA 8% + Acrylic 4% + Wax 0.5%"),
        sizing_pickup_pct: 11.5,
        creel_capacity: 672,
        tension_grams_per_end: 35.0,
    };
    lz4_roundtrip(&val, "warp beam preparation");
}

#[test]
fn test_weave_pattern_jacquard_lz4() {
    let warp = YarnSpecification {
        yarn_id: 200_001,
        fiber: FiberType::Silk,
        denier: 22.0,
        tex_count: 2.44,
        twist_per_meter: 500,
        twist_direction: String::from("S"),
        ply_count: 1,
        tenacity_cn_per_dtex: 4.0,
        elongation_pct: 20.0,
        moisture_regain_pct: 11.0,
    };
    let weft = YarnSpecification {
        yarn_id: 200_002,
        fiber: FiberType::Silk,
        denier: 30.0,
        tex_count: 3.33,
        twist_per_meter: 300,
        twist_direction: String::from("Z"),
        ply_count: 1,
        tenacity_cn_per_dtex: 3.8,
        elongation_pct: 22.0,
        moisture_regain_pct: 11.0,
    };
    let val = FabricDesignSpec {
        design_id: 90_001,
        pattern: WeavePattern::Jacquard {
            design_file: String::from("damask_rose_v3.jtl"),
            hooks: 2688,
        },
        warp_yarn: warp,
        weft_yarn: weft,
        epi: 120,
        ppi: 80,
        target_gsm: 85.0,
        target_width_cm: 140.0,
        crimp_warp_pct: 6.5,
        crimp_weft_pct: 3.2,
    };
    lz4_roundtrip(&val, "jacquard fabric design");
}

#[test]
fn test_weave_pattern_twill_lz4() {
    let warp = YarnSpecification {
        yarn_id: 300_001,
        fiber: FiberType::Cotton,
        denier: 200.0,
        tex_count: 22.2,
        twist_per_meter: 750,
        twist_direction: String::from("Z"),
        ply_count: 1,
        tenacity_cn_per_dtex: 3.2,
        elongation_pct: 8.0,
        moisture_regain_pct: 8.5,
    };
    let weft = YarnSpecification {
        yarn_id: 300_002,
        fiber: FiberType::Cotton,
        denier: 250.0,
        tex_count: 27.8,
        twist_per_meter: 650,
        twist_direction: String::from("Z"),
        ply_count: 1,
        tenacity_cn_per_dtex: 3.0,
        elongation_pct: 9.0,
        moisture_regain_pct: 8.5,
    };
    let val = FabricDesignSpec {
        design_id: 90_002,
        pattern: WeavePattern::Twill {
            direction: String::from("right-hand"),
            repeat: 4,
        },
        warp_yarn: warp,
        weft_yarn: weft,
        epi: 72,
        ppi: 56,
        target_gsm: 260.0,
        target_width_cm: 155.0,
        crimp_warp_pct: 9.0,
        crimp_weft_pct: 4.5,
    };
    lz4_roundtrip(&val, "twill fabric design");
}

#[test]
fn test_knitting_machine_params_lz4() {
    let val = KnittingMachineParams {
        machine_id: String::from("CK-30-24"),
        gauge: 28,
        diameter_inches: 30.0,
        feeder_count: 96,
        needle_count: 2640,
        speed_rpm: 28,
        stitch_length_mm: 2.7,
        yarn_tension_cn: 4.5,
        fabric_takedown_speed: 18.0,
    };
    lz4_roundtrip(&val, "knitting machine params");
}

#[test]
fn test_textile_test_batch_lz4() {
    let val = TextileTestBatch {
        batch_id: String::from("TB-2026-0315-A"),
        fabric_type: String::from("Nylon 6,6 Ripstop"),
        air_permeability_l_per_m2_s: 45.0,
        water_vapor_resistance: 3.8,
        thermal_resistance_clo: 0.22,
        abrasion_cycles_to_failure: 50_000,
        tear_strength_warp_n: 42.0,
        tear_strength_weft_n: 38.0,
        bursting_strength_kpa: 620.0,
        drape_coefficient: 0.68,
    };
    lz4_roundtrip(&val, "textile test batch");
}

#[test]
fn test_digital_print_job_lz4() {
    let val = DigitalPrintJob {
        job_id: 44_001,
        fabric_substrate: String::from("PES Satin 120gsm"),
        pretreatment: String::from("Cationic fixation pad"),
        passes: vec![
            PrintingPass {
                pass_number: 1,
                color_channel: String::from("Cyan"),
                ink_volume_ml_per_m2: 12.0,
                resolution_dpi: 1200,
                curing_temp_c: 170.0,
                curing_time_s: 30,
            },
            PrintingPass {
                pass_number: 2,
                color_channel: String::from("Magenta"),
                ink_volume_ml_per_m2: 10.5,
                resolution_dpi: 1200,
                curing_temp_c: 170.0,
                curing_time_s: 30,
            },
            PrintingPass {
                pass_number: 3,
                color_channel: String::from("Yellow"),
                ink_volume_ml_per_m2: 9.0,
                resolution_dpi: 1200,
                curing_temp_c: 170.0,
                curing_time_s: 30,
            },
            PrintingPass {
                pass_number: 4,
                color_channel: String::from("Black"),
                ink_volume_ml_per_m2: 8.0,
                resolution_dpi: 1200,
                curing_temp_c: 170.0,
                curing_time_s: 30,
            },
        ],
        total_area_m2: 250.0,
        color_gamut_coverage_pct: 92.5,
    };
    lz4_roundtrip(&val, "digital print job");
}

#[test]
fn test_finishing_heat_setting_polyester_lz4() {
    let val = FinishingSequence {
        sequence_id: 7_750,
        fabric_type: String::from("Polyester Chiffon"),
        treatments: vec![
            FinishingTreatment::HeatSetting {
                temperature_c: 190.0,
                duration_seconds: 25,
            },
            FinishingTreatment::Calendering {
                pressure_kn: 80.0,
                temperature_c: 180.0,
                nip_count: 2,
            },
            FinishingTreatment::WaterRepellent {
                fluorocarbon_free: true,
                rating: 4,
            },
        ],
        final_width_cm: 145.0,
        final_gsm: 72.0,
    };
    lz4_roundtrip(&val, "polyester finishing sequence");
}

#[test]
fn test_complex_fiber_blend_lz4() {
    let val = FiberBlendComposition {
        blend_id: String::from("BLD-PERF-4WAY"),
        components: vec![
            FiberBlendComponent {
                fiber: FiberType::Nylon,
                percentage: 45.0,
            },
            FiberBlendComponent {
                fiber: FiberType::Polyester,
                percentage: 30.0,
            },
            FiberBlendComponent {
                fiber: FiberType::Spandex,
                percentage: 15.0,
            },
            FiberBlendComponent {
                fiber: FiberType::Custom(String::from("Graphene-infused PA6")),
                percentage: 10.0,
            },
        ],
        intended_use: String::from("High-performance activewear"),
        trade_name: String::from("TechFlex Pro"),
    };
    lz4_roundtrip(&val, "complex fiber blend");
}

#[test]
fn test_vat_dyeing_bath_lz4() {
    let val = DyeingBathFormulation {
        bath_id: 55_302,
        dye_class: DyeClass::Vat,
        dye_name: String::from("Indanthrene Blue RSN"),
        concentration_g_per_l: 20.0,
        temperature_celsius: 50.0,
        ph_level: 12.5,
        duration_minutes: 30,
        salt_g_per_l: 0.0,
        alkali_g_per_l: 10.0,
        liquor_ratio: String::from("1:15"),
        auxiliaries: vec![
            String::from("Sodium hydrosulfite 25 g/L"),
            String::from("Dispersing agent 1 g/L"),
        ],
    };
    lz4_roundtrip(&val, "vat dyeing bath");
}

#[test]
fn test_empty_defect_report_clean_fabric_lz4() {
    let val = DefectReport {
        report_id: 8_900,
        fabric_roll_id: String::from("ROLL-2026-03-0200"),
        inspector_code: String::from("INS-12"),
        defects: vec![],
        total_length_inspected_m: 200.0,
        defect_points_per_100m: 0.0,
    };
    lz4_roundtrip(&val, "clean fabric defect report");
}
