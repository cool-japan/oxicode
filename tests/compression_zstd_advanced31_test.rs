//! Advanced Zstd compression tests for OxiCode — Flooring & Carpet Manufacturing domain.
//!
//! Covers encode -> compress -> decompress -> decode round-trips for types that
//! model real-world flooring and carpet manufacturing/installation data: carpet
//! fiber specifications, tufting machine parameters, dye lot tracking, hardwood
//! grading (NWFA), laminate click-lock dimensions, tile layout patterns, subfloor
//! moisture readings, installation work orders, warranty claim records, adhesive
//! application rates, seam placement calculations, underpad specifications,
//! commercial wear ratings, and more.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CarpetFiber {
    Nylon6,
    Nylon66,
    Polyester,
    Triexta,
    Polypropylene,
    Wool,
    WoolNylonBlend,
    Acrylic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TuftingPattern {
    LevelLoop,
    CutPile,
    CutLoop,
    Frieze,
    Saxony,
    Berber,
    Plush,
    Textured,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HardwoodGrade {
    Clear,
    Select,
    NumberOneCommon,
    NumberTwoCommon,
    CabinGrade,
    Utility,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TileLayoutPattern {
    StraightLay,
    DiagonalFortyFive,
    RunningBond,
    Herringbone,
    Chevron,
    Basketweave,
    Pinwheel,
    Versailles,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AdhesiveType {
    PressureSensitive,
    FullSpread,
    Urethane,
    Epoxy,
    AcrylicLatex,
    MoistureBarrier,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WarrantyClaimStatus {
    Filed,
    UnderReview,
    InspectionScheduled,
    InspectionComplete,
    Approved,
    Denied,
    PaidOut,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SubfloorType {
    ConcreteOnGrade,
    ConcreteSuspended,
    PlywoodOverJoist,
    OsbOverJoist,
    GypsumUnderlayment,
    ExistingVinyl,
    ExistingTile,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WearRatingClass {
    Residential21,
    Residential22,
    Residential23,
    Commercial31,
    Commercial32,
    Commercial33,
    Industrial41,
    Industrial42,
    Industrial43,
}

/// Carpet fiber specification for a production run.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarpetFiberSpec {
    spec_id: u64,
    fiber: CarpetFiber,
    denier_per_filament: u32,
    filament_count: u16,
    ply_twist_per_inch: u16,
    heat_set_temperature_c: u16,
    stain_resist_level: u8,
    static_control: bool,
    antimicrobial_treated: bool,
    color_name: String,
}

/// Tufting machine operating parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TuftingMachineParams {
    machine_id: u32,
    gauge_fraction_inch: u16,
    stitch_rate_per_inch: u16,
    pile_height_thousandths_inch: u32,
    pattern: TuftingPattern,
    needle_count: u16,
    backing_weight_oz_sqyd: u32,
    face_weight_oz_sqyd: u32,
    production_speed_ft_per_min: u16,
    yarn_feed_tension_grams: u16,
}

/// Dye lot tracking for carpet production batches.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DyeLotRecord {
    lot_number: u64,
    color_code: String,
    dye_method: String,
    batch_size_sqyd: u32,
    ph_level_x100: u16,
    temperature_c_x10: u16,
    dwell_time_seconds: u32,
    lightfastness_rating: u8,
    colorfastness_wet: u8,
    colorfastness_dry: u8,
    delta_e_tolerance_x100: u16,
    samples: Vec<u16>,
}

/// NWFA hardwood grading report for a delivery of planks.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HardwoodGradingReport {
    report_id: u64,
    species: String,
    grade: HardwoodGrade,
    moisture_content_pct_x10: u16,
    janka_hardness_lbf: u16,
    board_count: u32,
    total_sqft_x100: u64,
    avg_width_thousandths: u32,
    avg_length_thousandths: u32,
    avg_thickness_thousandths: u32,
    defect_counts: Vec<u32>,
}

/// Laminate click-lock panel dimensional data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LaminateClickLockDimension {
    panel_sku: String,
    length_mm_x10: u32,
    width_mm_x10: u32,
    thickness_mm_x100: u16,
    wear_layer_mm_x100: u16,
    tongue_depth_mm_x100: u16,
    groove_depth_mm_x100: u16,
    click_angle_degrees_x10: u16,
    ac_rating: u8,
    swelling_pct_x100: u16,
    impact_resistance_class: u8,
}

/// Tile layout plan for a room.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TileLayoutPlan {
    room_id: u32,
    pattern: TileLayoutPattern,
    tile_width_mm: u16,
    tile_height_mm: u16,
    grout_spacing_mm_x10: u16,
    room_length_mm: u32,
    room_width_mm: u32,
    cut_tiles_count: u32,
    full_tiles_count: u32,
    waste_pct_x100: u16,
    accent_tile_positions: Vec<(u16, u16)>,
}

/// Subfloor moisture reading log.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubfloorMoistureLog {
    job_id: u64,
    subfloor: SubfloorType,
    readings: Vec<SubfloorReading>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubfloorReading {
    grid_x: u16,
    grid_y: u16,
    moisture_pct_x100: u16,
    relative_humidity_x100: u16,
    temperature_c_x10: i16,
    passes_threshold: bool,
}

/// Installation work order.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InstallationWorkOrder {
    order_id: u64,
    customer_id: u64,
    installer_id: u32,
    scheduled_date_unix: u64,
    estimated_hours_x10: u16,
    room_count: u8,
    total_sqft_x100: u64,
    material_sku_list: Vec<String>,
    notes: String,
    requires_furniture_move: bool,
    requires_old_floor_removal: bool,
    stair_count: u16,
}

/// Warranty claim record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WarrantyClaimRecord {
    claim_id: u64,
    order_id: u64,
    status: WarrantyClaimStatus,
    filed_date_unix: u64,
    product_sku: String,
    installed_sqft_x100: u64,
    defect_description: String,
    inspector_findings: String,
    claim_amount_cents: u64,
    resolution_code: u16,
}

/// Adhesive application rate specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdhesiveApplicationRate {
    adhesive_sku: String,
    adhesive_type: AdhesiveType,
    trowel_notch_size_inches_x100: u16,
    coverage_sqft_per_gal_x10: u16,
    open_time_minutes: u16,
    working_time_minutes: u16,
    cure_time_hours: u16,
    min_temp_f_x10: i16,
    max_temp_f_x10: i16,
    max_moisture_pct_x100: u16,
    voc_grams_per_liter: u16,
}

/// Carpet seam placement calculation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeamPlacementPlan {
    room_id: u32,
    carpet_roll_width_inches: u16,
    room_length_inches_x10: u32,
    room_width_inches_x10: u32,
    seam_count: u8,
    seam_positions: Vec<SeamLine>,
    total_waste_sqft_x100: u32,
    pile_direction_degrees: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeamLine {
    start_x_inches_x10: u32,
    start_y_inches_x10: u32,
    end_x_inches_x10: u32,
    end_y_inches_x10: u32,
    seam_type: u8,
}

/// Underpad (carpet cushion) specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UnderpadSpec {
    product_sku: String,
    material: String,
    thickness_inches_x1000: u16,
    density_lb_per_cuft_x10: u16,
    compression_resistance_psi_x10: u16,
    thermal_resistance_r_value_x100: u16,
    moisture_barrier: bool,
    antimicrobial: bool,
    recycled_content_pct: u8,
    roll_width_ft: u8,
    roll_length_ft: u16,
}

/// Commercial flooring wear rating assessment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommercialWearAssessment {
    assessment_id: u64,
    product_sku: String,
    wear_class: WearRatingClass,
    taber_cycles: u32,
    delta_gloss_x100: u16,
    residual_indent_mm_x1000: u16,
    castor_chair_passes: u32,
    stain_resistance_class: u8,
    chemical_resistance_class: u8,
    slip_resistance_r_value: u8,
    fire_class: String,
}

/// Radiant heat flooring compatibility test.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiantHeatCompat {
    product_sku: String,
    max_surface_temp_f_x10: u16,
    thermal_resistance_r_x1000: u16,
    dimensional_stability_pct_x1000: u16,
    heat_ramp_rate_f_per_hour_x10: u16,
    cool_down_hours_x10: u16,
    approved: bool,
    test_cycle_count: u32,
    delamination_observed: bool,
    gap_measurement_mm_x1000: Vec<u16>,
}

/// Production quality control sample for carpet rolls.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarpetQcSample {
    roll_id: u64,
    pile_height_measurements_x1000: Vec<u16>,
    backing_peel_strength_x100: Vec<u16>,
    tuft_bind_oz_x10: Vec<u16>,
    weight_per_sqyd_oz_x100: Vec<u32>,
    colorfastness_gray_scale: u8,
    visual_defect_count: u16,
    pass: bool,
}

/// Staircase flooring installation specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StaircaseInstallSpec {
    staircase_id: u32,
    tread_count: u8,
    riser_count: u8,
    tread_depth_inches_x100: u16,
    riser_height_inches_x100: u16,
    nosing_overhang_inches_x100: u16,
    landing_sqft_x100: u32,
    material_sku: String,
    bullnose_type: String,
    needs_anti_slip: bool,
    step_measurements: Vec<(u16, u16)>,
}

/// Flooring material acclimation log.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AcclimationLog {
    job_id: u64,
    material_sku: String,
    start_timestamp_unix: u64,
    end_timestamp_unix: u64,
    room_temp_readings_f_x10: Vec<i16>,
    room_humidity_readings_x10: Vec<u16>,
    material_moisture_readings_x100: Vec<u16>,
    target_moisture_pct_x100: u16,
    acclimation_met: bool,
}

/// Inventory management for a flooring warehouse.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WarehouseInventoryEntry {
    sku: String,
    lot_number: u64,
    quantity_sqft_x100: u64,
    warehouse_zone: String,
    rack_position: u32,
    received_date_unix: u64,
    expiry_date_unix: u64,
    unit_cost_cents: u64,
    weight_lbs_x100: u32,
    pallet_count: u16,
}

/// Transition strip specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransitionStripSpec {
    strip_sku: String,
    strip_type: String,
    material: String,
    length_inches_x10: u32,
    width_inches_x100: u16,
    height_diff_accommodated_x100: u16,
    color_code: String,
    mounting_method: String,
    track_included: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_carpet_fiber_spec(id: u64, fiber: CarpetFiber) -> CarpetFiberSpec {
    CarpetFiberSpec {
        spec_id: id,
        fiber,
        denier_per_filament: 15 + (id as u32 % 20),
        filament_count: 48 + (id as u16 % 100),
        ply_twist_per_inch: 4 + (id as u16 % 6),
        heat_set_temperature_c: 120 + (id as u16 % 30),
        stain_resist_level: 3 + (id as u8 % 5),
        static_control: id % 2 == 0,
        antimicrobial_treated: id % 3 == 0,
        color_name: format!("DyeColor-{id}"),
    }
}

fn make_subfloor_reading(x: u16, y: u16) -> SubfloorReading {
    SubfloorReading {
        grid_x: x,
        grid_y: y,
        moisture_pct_x100: 300 + (x as u16 * 7 + y as u16 * 13) % 500,
        relative_humidity_x100: 4500 + (x as u16 * 11 + y as u16 * 3) % 2000,
        temperature_c_x10: 200 + (x as i16 * 3 + y as i16 * 2),
        passes_threshold: ((x + y) % 5) != 0,
    }
}

fn make_seam_line(idx: u8) -> SeamLine {
    SeamLine {
        start_x_inches_x10: idx as u32 * 1440,
        start_y_inches_x10: 0,
        end_x_inches_x10: idx as u32 * 1440,
        end_y_inches_x10: 2400,
        seam_type: idx % 2,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Round-trip for carpet fiber specification — nylon 6,6 high-denier.
#[test]
fn test_zstd_carpet_fiber_spec_nylon66_roundtrip() {
    let spec = make_carpet_fiber_spec(1001, CarpetFiber::Nylon66);
    let encoded = encode_to_vec(&spec).expect("encode CarpetFiberSpec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (CarpetFiberSpec, usize) =
        decode_from_slice(&decompressed).expect("decode CarpetFiberSpec failed");
    assert_eq!(spec, decoded);
}

/// 2. Round-trip for a batch of carpet fiber specs across different fiber types.
#[test]
fn test_zstd_carpet_fiber_spec_batch_roundtrip() {
    let fibers = [
        CarpetFiber::Nylon6,
        CarpetFiber::Polyester,
        CarpetFiber::Wool,
        CarpetFiber::Triexta,
        CarpetFiber::Polypropylene,
        CarpetFiber::Acrylic,
        CarpetFiber::WoolNylonBlend,
    ];
    let batch: Vec<CarpetFiberSpec> = fibers
        .iter()
        .enumerate()
        .map(|(i, f)| make_carpet_fiber_spec(i as u64 + 2000, f.clone()))
        .collect();
    let encoded = encode_to_vec(&batch).expect("encode Vec<CarpetFiberSpec> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CarpetFiberSpec>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CarpetFiberSpec> failed");
    assert_eq!(batch, decoded);
}

/// 3. Round-trip for tufting machine parameters.
#[test]
fn test_zstd_tufting_machine_params_roundtrip() {
    let params = TuftingMachineParams {
        machine_id: 42,
        gauge_fraction_inch: 10,
        stitch_rate_per_inch: 8,
        pile_height_thousandths_inch: 750,
        pattern: TuftingPattern::Saxony,
        needle_count: 1200,
        backing_weight_oz_sqyd: 56,
        face_weight_oz_sqyd: 40,
        production_speed_ft_per_min: 45,
        yarn_feed_tension_grams: 120,
    };
    let encoded = encode_to_vec(&params).expect("encode TuftingMachineParams failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TuftingMachineParams, usize) =
        decode_from_slice(&decompressed).expect("decode TuftingMachineParams failed");
    assert_eq!(params, decoded);
}

/// 4. Round-trip for dye lot records with spectrophotometer samples.
#[test]
fn test_zstd_dye_lot_record_roundtrip() {
    let record = DyeLotRecord {
        lot_number: 20260315_0001,
        color_code: "BC-4429-MIDNIGHT".to_string(),
        dye_method: "ContinuousBeckDyeing".to_string(),
        batch_size_sqyd: 15000,
        ph_level_x100: 650,
        temperature_c_x10: 980,
        dwell_time_seconds: 3600,
        lightfastness_rating: 5,
        colorfastness_wet: 4,
        colorfastness_dry: 5,
        delta_e_tolerance_x100: 50,
        samples: (0u16..64).map(|i| 400 + i * 3).collect(),
    };
    let encoded = encode_to_vec(&record).expect("encode DyeLotRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (DyeLotRecord, usize) =
        decode_from_slice(&decompressed).expect("decode DyeLotRecord failed");
    assert_eq!(record, decoded);
}

/// 5. Round-trip for NWFA hardwood grading report.
#[test]
fn test_zstd_hardwood_grading_report_roundtrip() {
    let report = HardwoodGradingReport {
        report_id: 88001,
        species: "WhiteOak".to_string(),
        grade: HardwoodGrade::Select,
        moisture_content_pct_x10: 72,
        janka_hardness_lbf: 1360,
        board_count: 480,
        total_sqft_x100: 192000,
        avg_width_thousandths: 5000,
        avg_length_thousandths: 84000,
        avg_thickness_thousandths: 750,
        defect_counts: vec![2, 0, 5, 1, 0, 3, 7, 0, 0, 1],
    };
    let encoded = encode_to_vec(&report).expect("encode HardwoodGradingReport failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (HardwoodGradingReport, usize) =
        decode_from_slice(&decompressed).expect("decode HardwoodGradingReport failed");
    assert_eq!(report, decoded);
}

/// 6. Round-trip for laminate click-lock panel dimensions.
#[test]
fn test_zstd_laminate_click_lock_roundtrip() {
    let panel = LaminateClickLockDimension {
        panel_sku: "LVP-2260-ASHWOOD".to_string(),
        length_mm_x10: 12200,
        width_mm_x10: 1810,
        thickness_mm_x100: 800,
        wear_layer_mm_x100: 50,
        tongue_depth_mm_x100: 35,
        groove_depth_mm_x100: 37,
        click_angle_degrees_x10: 200,
        ac_rating: 4,
        swelling_pct_x100: 1200,
        impact_resistance_class: 2,
    };
    let encoded = encode_to_vec(&panel).expect("encode LaminateClickLockDimension failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (LaminateClickLockDimension, usize) =
        decode_from_slice(&decompressed).expect("decode LaminateClickLockDimension failed");
    assert_eq!(panel, decoded);
}

/// 7. Round-trip for tile layout plan with accent tile positions.
#[test]
fn test_zstd_tile_layout_plan_roundtrip() {
    let plan = TileLayoutPlan {
        room_id: 7,
        pattern: TileLayoutPattern::Herringbone,
        tile_width_mm: 600,
        tile_height_mm: 100,
        grout_spacing_mm_x10: 30,
        room_length_mm: 5000,
        room_width_mm: 4000,
        cut_tiles_count: 34,
        full_tiles_count: 280,
        waste_pct_x100: 1200,
        accent_tile_positions: (0u16..12).map(|i| (i * 3, i * 2 + 1)).collect(),
    };
    let encoded = encode_to_vec(&plan).expect("encode TileLayoutPlan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TileLayoutPlan, usize) =
        decode_from_slice(&decompressed).expect("decode TileLayoutPlan failed");
    assert_eq!(plan, decoded);
}

/// 8. Round-trip for subfloor moisture reading grid.
#[test]
fn test_zstd_subfloor_moisture_log_roundtrip() {
    let readings: Vec<SubfloorReading> = (0u16..8)
        .flat_map(|x| (0u16..6).map(move |y| make_subfloor_reading(x, y)))
        .collect();
    let log = SubfloorMoistureLog {
        job_id: 550123,
        subfloor: SubfloorType::ConcreteOnGrade,
        readings,
    };
    let encoded = encode_to_vec(&log).expect("encode SubfloorMoistureLog failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SubfloorMoistureLog, usize) =
        decode_from_slice(&decompressed).expect("decode SubfloorMoistureLog failed");
    assert_eq!(log, decoded);
}

/// 9. Round-trip for installation work order.
#[test]
fn test_zstd_installation_work_order_roundtrip() {
    let order = InstallationWorkOrder {
        order_id: 900100,
        customer_id: 70042,
        installer_id: 315,
        scheduled_date_unix: 1773676800,
        estimated_hours_x10: 65,
        room_count: 4,
        total_sqft_x100: 125000,
        material_sku_list: vec![
            "HW-OAK-SELECT-5IN".to_string(),
            "UND-PREMIUM-6MM".to_string(),
            "TRANS-T-MOLD-OAK".to_string(),
            "ADH-URE-3GAL".to_string(),
        ],
        notes: "Customer requests diagonal layout in living room. Dogs on-site.".to_string(),
        requires_furniture_move: true,
        requires_old_floor_removal: true,
        stair_count: 14,
    };
    let encoded = encode_to_vec(&order).expect("encode InstallationWorkOrder failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (InstallationWorkOrder, usize) =
        decode_from_slice(&decompressed).expect("decode InstallationWorkOrder failed");
    assert_eq!(order, decoded);
}

/// 10. Round-trip for warranty claim record.
#[test]
fn test_zstd_warranty_claim_record_roundtrip() {
    let claim = WarrantyClaimRecord {
        claim_id: 40001,
        order_id: 900100,
        status: WarrantyClaimStatus::InspectionComplete,
        filed_date_unix: 1774000000,
        product_sku: "LVP-2260-ASHWOOD".to_string(),
        installed_sqft_x100: 85000,
        defect_description: "Edge curling observed in 3 planks near exterior wall after 6 months"
            .to_string(),
        inspector_findings:
            "Moisture ingress from sliding door. Subfloor readings 5.2% at defect location."
                .to_string(),
        claim_amount_cents: 234500,
        resolution_code: 3,
    };
    let encoded = encode_to_vec(&claim).expect("encode WarrantyClaimRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (WarrantyClaimRecord, usize) =
        decode_from_slice(&decompressed).expect("decode WarrantyClaimRecord failed");
    assert_eq!(claim, decoded);
}

/// 11. Round-trip for adhesive application rate spec.
#[test]
fn test_zstd_adhesive_application_rate_roundtrip() {
    let rate = AdhesiveApplicationRate {
        adhesive_sku: "ADH-URE-PREMIUM-5GAL".to_string(),
        adhesive_type: AdhesiveType::Urethane,
        trowel_notch_size_inches_x100: 25,
        coverage_sqft_per_gal_x10: 550,
        open_time_minutes: 60,
        working_time_minutes: 45,
        cure_time_hours: 24,
        min_temp_f_x10: 650,
        max_temp_f_x10: 950,
        max_moisture_pct_x100: 300,
        voc_grams_per_liter: 35,
    };
    let encoded = encode_to_vec(&rate).expect("encode AdhesiveApplicationRate failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AdhesiveApplicationRate, usize) =
        decode_from_slice(&decompressed).expect("decode AdhesiveApplicationRate failed");
    assert_eq!(rate, decoded);
}

/// 12. Round-trip for carpet seam placement plan.
#[test]
fn test_zstd_seam_placement_plan_roundtrip() {
    let plan = SeamPlacementPlan {
        room_id: 3,
        carpet_roll_width_inches: 144,
        room_length_inches_x10: 2400,
        room_width_inches_x10: 3600,
        seam_count: 2,
        seam_positions: (1u8..=2).map(make_seam_line).collect(),
        total_waste_sqft_x100: 5400,
        pile_direction_degrees: 90,
    };
    let encoded = encode_to_vec(&plan).expect("encode SeamPlacementPlan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SeamPlacementPlan, usize) =
        decode_from_slice(&decompressed).expect("decode SeamPlacementPlan failed");
    assert_eq!(plan, decoded);
}

/// 13. Round-trip for underpad specification.
#[test]
fn test_zstd_underpad_spec_roundtrip() {
    let spec = UnderpadSpec {
        product_sku: "UND-REBOND-8LB-716".to_string(),
        material: "BondedUrethaneRebond".to_string(),
        thickness_inches_x1000: 438,
        density_lb_per_cuft_x10: 80,
        compression_resistance_psi_x10: 65,
        thermal_resistance_r_value_x100: 120,
        moisture_barrier: true,
        antimicrobial: true,
        recycled_content_pct: 85,
        roll_width_ft: 6,
        roll_length_ft: 90,
    };
    let encoded = encode_to_vec(&spec).expect("encode UnderpadSpec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (UnderpadSpec, usize) =
        decode_from_slice(&decompressed).expect("decode UnderpadSpec failed");
    assert_eq!(spec, decoded);
}

/// 14. Round-trip for commercial wear rating assessment.
#[test]
fn test_zstd_commercial_wear_assessment_roundtrip() {
    let assessment = CommercialWearAssessment {
        assessment_id: 60010,
        product_sku: "LVT-COMM-SLATE-4MM".to_string(),
        wear_class: WearRatingClass::Commercial33,
        taber_cycles: 8000,
        delta_gloss_x100: 350,
        residual_indent_mm_x1000: 80,
        castor_chair_passes: 25000,
        stain_resistance_class: 4,
        chemical_resistance_class: 3,
        slip_resistance_r_value: 10,
        fire_class: "Bfl-s1".to_string(),
    };
    let encoded = encode_to_vec(&assessment).expect("encode CommercialWearAssessment failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (CommercialWearAssessment, usize) =
        decode_from_slice(&decompressed).expect("decode CommercialWearAssessment failed");
    assert_eq!(assessment, decoded);
}

/// 15. Round-trip for radiant heat compatibility test results.
#[test]
fn test_zstd_radiant_heat_compat_roundtrip() {
    let compat = RadiantHeatCompat {
        product_sku: "ENG-HW-MAPLE-5IN".to_string(),
        max_surface_temp_f_x10: 850,
        thermal_resistance_r_x1000: 370,
        dimensional_stability_pct_x1000: 15,
        heat_ramp_rate_f_per_hour_x10: 50,
        cool_down_hours_x10: 240,
        approved: true,
        test_cycle_count: 500,
        delamination_observed: false,
        gap_measurement_mm_x1000: (0u16..20).map(|i| 10 + i * 2).collect(),
    };
    let encoded = encode_to_vec(&compat).expect("encode RadiantHeatCompat failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (RadiantHeatCompat, usize) =
        decode_from_slice(&decompressed).expect("decode RadiantHeatCompat failed");
    assert_eq!(compat, decoded);
}

/// 16. Round-trip for carpet QC sample with multiple measurement vectors.
#[test]
fn test_zstd_carpet_qc_sample_roundtrip() {
    let sample = CarpetQcSample {
        roll_id: 777001,
        pile_height_measurements_x1000: (0u16..30).map(|i| 720 + (i % 5) * 2).collect(),
        backing_peel_strength_x100: (0u16..10).map(|i| 350 + i * 10).collect(),
        tuft_bind_oz_x10: (0u16..10).map(|i| 120 + i * 5).collect(),
        weight_per_sqyd_oz_x100: (0u32..10).map(|i| 4200 + i * 20).collect(),
        colorfastness_gray_scale: 4,
        visual_defect_count: 1,
        pass: true,
    };
    let encoded = encode_to_vec(&sample).expect("encode CarpetQcSample failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (CarpetQcSample, usize) =
        decode_from_slice(&decompressed).expect("decode CarpetQcSample failed");
    assert_eq!(sample, decoded);
}

/// 17. Round-trip for staircase installation specification.
#[test]
fn test_zstd_staircase_install_spec_roundtrip() {
    let spec = StaircaseInstallSpec {
        staircase_id: 5,
        tread_count: 14,
        riser_count: 15,
        tread_depth_inches_x100: 1100,
        riser_height_inches_x100: 750,
        nosing_overhang_inches_x100: 100,
        landing_sqft_x100: 1600,
        material_sku: "HW-OAK-SELECT-STAIR".to_string(),
        bullnose_type: "FullRound".to_string(),
        needs_anti_slip: true,
        step_measurements: (0u16..14).map(|i| (1100 + i % 3, 750 + i % 2)).collect(),
    };
    let encoded = encode_to_vec(&spec).expect("encode StaircaseInstallSpec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (StaircaseInstallSpec, usize) =
        decode_from_slice(&decompressed).expect("decode StaircaseInstallSpec failed");
    assert_eq!(spec, decoded);
}

/// 18. Round-trip for material acclimation log with time-series readings.
#[test]
fn test_zstd_acclimation_log_roundtrip() {
    let log = AcclimationLog {
        job_id: 550200,
        material_sku: "ENG-HW-WALNUT-7IN".to_string(),
        start_timestamp_unix: 1773500000,
        end_timestamp_unix: 1773759600,
        room_temp_readings_f_x10: (0i16..72).map(|h| 690 + (h % 8) * 5).collect(),
        room_humidity_readings_x10: (0u16..72).map(|h| 420 + (h % 10) * 3).collect(),
        material_moisture_readings_x100: (0u16..72).map(|h| 850 - h * 2).collect(),
        target_moisture_pct_x100: 700,
        acclimation_met: true,
    };
    let encoded = encode_to_vec(&log).expect("encode AcclimationLog failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AcclimationLog, usize) =
        decode_from_slice(&decompressed).expect("decode AcclimationLog failed");
    assert_eq!(log, decoded);
}

/// 19. Round-trip for warehouse inventory entries.
#[test]
fn test_zstd_warehouse_inventory_batch_roundtrip() {
    let entries: Vec<WarehouseInventoryEntry> = (0u64..25)
        .map(|i| WarehouseInventoryEntry {
            sku: format!("SKU-FLR-{:04}", 1000 + i),
            lot_number: 20260300 + i,
            quantity_sqft_x100: 50000 + i * 1200,
            warehouse_zone: format!("Zone-{}", (b'A' + (i % 4) as u8) as char),
            rack_position: 100 + i as u32 * 3,
            received_date_unix: 1773000000 + i * 86400,
            expiry_date_unix: 1804536000 + i * 86400,
            unit_cost_cents: 350 + i * 15,
            weight_lbs_x100: 28000 + i as u32 * 500,
            pallet_count: 2 + (i % 5) as u16,
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode Vec<WarehouseInventoryEntry> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<WarehouseInventoryEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<WarehouseInventoryEntry> failed");
    assert_eq!(entries, decoded);
}

/// 20. Round-trip for transition strip specifications.
#[test]
fn test_zstd_transition_strip_spec_roundtrip() {
    let strips: Vec<TransitionStripSpec> = vec![
        TransitionStripSpec {
            strip_sku: "TR-TMOLD-OAK-72".to_string(),
            strip_type: "TMolding".to_string(),
            material: "SolidHardwood".to_string(),
            length_inches_x10: 720,
            width_inches_x100: 200,
            height_diff_accommodated_x100: 0,
            color_code: "OAK-NAT-220".to_string(),
            mounting_method: "TrackSnap".to_string(),
            track_included: true,
        },
        TransitionStripSpec {
            strip_sku: "TR-REDUCER-WAL-72".to_string(),
            strip_type: "Reducer".to_string(),
            material: "EngineeredWood".to_string(),
            length_inches_x10: 720,
            width_inches_x100: 250,
            height_diff_accommodated_x100: 38,
            color_code: "WAL-DK-340".to_string(),
            mounting_method: "AdhesiveBond".to_string(),
            track_included: false,
        },
        TransitionStripSpec {
            strip_sku: "TR-THRESH-ALU-36".to_string(),
            strip_type: "Threshold".to_string(),
            material: "Aluminum".to_string(),
            length_inches_x10: 360,
            width_inches_x100: 350,
            height_diff_accommodated_x100: 50,
            color_code: "ALU-SATIN-100".to_string(),
            mounting_method: "ScrewDown".to_string(),
            track_included: false,
        },
    ];
    let encoded = encode_to_vec(&strips).expect("encode Vec<TransitionStripSpec> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TransitionStripSpec>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<TransitionStripSpec> failed");
    assert_eq!(strips, decoded);
}

/// 21. Round-trip for multiple warranty claims across different statuses.
#[test]
fn test_zstd_warranty_claims_multi_status_roundtrip() {
    let statuses = [
        WarrantyClaimStatus::Filed,
        WarrantyClaimStatus::UnderReview,
        WarrantyClaimStatus::InspectionScheduled,
        WarrantyClaimStatus::InspectionComplete,
        WarrantyClaimStatus::Approved,
        WarrantyClaimStatus::Denied,
        WarrantyClaimStatus::PaidOut,
    ];
    let claims: Vec<WarrantyClaimRecord> = statuses
        .iter()
        .enumerate()
        .map(|(i, status)| WarrantyClaimRecord {
            claim_id: 50000 + i as u64,
            order_id: 800000 + i as u64 * 100,
            status: status.clone(),
            filed_date_unix: 1774000000 + i as u64 * 604800,
            product_sku: format!("PROD-{:04}", 3000 + i),
            installed_sqft_x100: 60000 + i as u64 * 5000,
            defect_description: format!("Defect type {} observed in high-traffic area", i + 1),
            inspector_findings: format!("Finding set {} — root cause analysis pending", i + 1),
            claim_amount_cents: 15000 + i as u64 * 7500,
            resolution_code: i as u16,
        })
        .collect();
    let encoded = encode_to_vec(&claims).expect("encode Vec<WarrantyClaimRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<WarrantyClaimRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<WarrantyClaimRecord> failed");
    assert_eq!(claims, decoded);
}

/// 22. Round-trip for a combined flooring project: subfloor log + work order + adhesive.
#[test]
fn test_zstd_combined_flooring_project_roundtrip() {
    let moisture_log = SubfloorMoistureLog {
        job_id: 990001,
        subfloor: SubfloorType::PlywoodOverJoist,
        readings: (0u16..5)
            .flat_map(|x| (0u16..4).map(move |y| make_subfloor_reading(x, y)))
            .collect(),
    };
    let work_order = InstallationWorkOrder {
        order_id: 990001,
        customer_id: 88010,
        installer_id: 200,
        scheduled_date_unix: 1773849600,
        estimated_hours_x10: 80,
        room_count: 3,
        total_sqft_x100: 98000,
        material_sku_list: vec![
            "LVP-2260-ASHWOOD".to_string(),
            "UND-REBOND-8LB-716".to_string(),
        ],
        notes: "Acclimation verified. Subfloor approved.".to_string(),
        requires_furniture_move: false,
        requires_old_floor_removal: false,
        stair_count: 0,
    };
    let adhesive = AdhesiveApplicationRate {
        adhesive_sku: "ADH-PS-CARPET-4GAL".to_string(),
        adhesive_type: AdhesiveType::PressureSensitive,
        trowel_notch_size_inches_x100: 16,
        coverage_sqft_per_gal_x10: 700,
        open_time_minutes: 90,
        working_time_minutes: 60,
        cure_time_hours: 12,
        min_temp_f_x10: 600,
        max_temp_f_x10: 900,
        max_moisture_pct_x100: 350,
        voc_grams_per_liter: 20,
    };
    let project = (moisture_log, work_order, adhesive);
    let encoded = encode_to_vec(&project).expect("encode combined project tuple failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            SubfloorMoistureLog,
            InstallationWorkOrder,
            AdhesiveApplicationRate,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode combined project tuple failed");
    assert_eq!(project, decoded);
}
