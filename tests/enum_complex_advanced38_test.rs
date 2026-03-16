//! Advanced complex enum encoding tests for OxiCode - set 38
//! Theme: Jewelry manufacturing and gemological certification.
//! 22 test functions covering gemstone grading, precious metals, settings,
//! certification bodies, repair work orders, CAD parameters, and more.

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

// --- Diamond 4Cs grading ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum DiamondCut {
    Ideal,
    Excellent,
    VeryGood,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DiamondClarity {
    Flawless,
    InternallyFlawless,
    VVS1,
    VVS2,
    VS1,
    VS2,
    SI1,
    SI2,
    I1,
    I2,
    I3,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DiamondColor {
    Colorless(String),
    NearColorless(String),
    Faint(String),
    VeryLight(String),
    Light(String),
    FancyVivid { hue: String, saturation: u8 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DiamondGrading {
    carat_weight_milligrams: u32,
    cut: DiamondCut,
    clarity: DiamondClarity,
    color: DiamondColor,
    fluorescence: Option<String>,
}

// --- Colored stone grading ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ColoredStoneType {
    Ruby,
    Sapphire {
        color_modifier: String,
    },
    Emerald,
    Alexandrite,
    Tanzanite,
    Paraiba,
    Spinel(String),
    Garnet {
        variety: String,
        chemical_group: String,
    },
    Tourmaline(String),
    Opal {
        play_of_color: bool,
        body_tone: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColoredStoneGrading {
    stone_type: ColoredStoneType,
    carat_weight_milligrams: u32,
    hue: String,
    tone: u8,
    saturation: u8,
    clarity_type: String,
    origin_country: Option<String>,
}

// --- Precious metal alloys ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum PreciousMetalAlloy {
    Gold {
        karat: u8,
        color: String,
        alloy_composition_ppm: Vec<u32>,
    },
    Platinum {
        purity_ppt: u16,
        iridium_ppm: u32,
    },
    Palladium {
        purity_ppt: u16,
    },
    Silver {
        purity_ppt: u16,
        anti_tarnish: bool,
    },
    TwoTone {
        primary: String,
        secondary: String,
        primary_purity_ppt: u16,
    },
}

// --- Setting types ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SettingType {
    Prong {
        prong_count: u8,
        prong_shape: String,
    },
    Bezel {
        full: bool,
    },
    Channel {
        stone_count: u8,
    },
    Pave {
        micro_pave: bool,
        stone_diameter_um: u32,
    },
    Tension,
    Flush,
    Bar {
        spacing_um: u32,
    },
    Invisible {
        grid_rows: u8,
        grid_cols: u8,
    },
    Cluster {
        arrangement: String,
        center_stone_mm: u32,
    },
    CathedralArch {
        height_mm: u16,
    },
}

// --- Hallmark stamps ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum HallmarkStamp {
    PurityMark {
        metal: String,
        purity_value: String,
    },
    MakersMark(String),
    AssayOfficeMark {
        office: String,
        year: u16,
    },
    DateLetter(char),
    DutyMark,
    ImportMark {
        country_of_origin: String,
        importing_office: String,
    },
    LaserEngraved {
        text: String,
        font_size_um: u32,
    },
}

// --- Certification bodies ---

#[derive(Debug, PartialEq, Encode, Decode)]
#[allow(clippy::upper_case_acronyms)]
enum CertificationBody {
    GIA {
        report_number: u64,
        report_type: String,
    },
    AGS {
        certificate_id: String,
        ideal_scope: bool,
    },
    IGI {
        report_number: u64,
    },
    HRD {
        certificate_id: String,
    },
    GCAL {
        laser_inscription: String,
    },
    Gubelin {
        origin_report: bool,
        appendix_count: u8,
    },
    SSEF {
        advanced_testing: Vec<String>,
    },
    Independent {
        lab_name: String,
        accreditation: String,
    },
}

// --- Repair work orders ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum RepairWorkOrder {
    Sizing {
        current_size_tenth_mm: u16,
        target_size_tenth_mm: u16,
    },
    ProngRetipping {
        prong_indices: Vec<u8>,
        metal: String,
    },
    StoneReplacement {
        position: String,
        original_stone: String,
        replacement_stone: String,
        carat_milligrams: u32,
    },
    Soldering {
        joint_locations: Vec<String>,
        solder_type: String,
    },
    Refinishing {
        polish_grade: String,
        rhodium_plate: bool,
    },
    ClaspReplacement {
        clasp_type: String,
        length_mm: u16,
    },
    Engraving {
        text: String,
        font: String,
        depth_um: u16,
    },
    LaserWelding {
        weld_points: Vec<(u32, u32)>,
        power_watts: u16,
    },
}

// --- Custom design CAD parameters ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum CadDesignParameter {
    RingProfile {
        width_um: u32,
        thickness_um: u32,
        comfort_fit: bool,
        profile_shape: String,
    },
    PendantOutline {
        outline_points: Vec<(i32, i32)>,
        bail_type: String,
        bail_diameter_um: u32,
    },
    EarringPost {
        post_gauge: u8,
        post_length_um: u32,
        backing_type: String,
    },
    BraceletLink {
        link_length_um: u32,
        link_width_um: u32,
        link_count: u16,
        clasp_style: String,
    },
    BezierPath {
        control_points: Vec<(i32, i32)>,
        resolution: u16,
    },
}

// --- Origin certification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum StoneOrigin {
    Natural {
        mine_name: Option<String>,
        country: String,
        region: Option<String>,
    },
    LabGrown {
        method: String,
        manufacturer: String,
        seed_crystal: Option<String>,
    },
    Treated {
        base_origin: String,
        treatments: Vec<String>,
        permanence: String,
    },
    Synthetic {
        process: String,
        year_produced: u16,
    },
    Unknown,
}

// --- Appraisal valuations ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AppraisalPurpose {
    InsuranceReplacement,
    FairMarketValue,
    LiquidationValue,
    EstateSettlement,
    DamageAssessment {
        pre_damage_value_cents: u64,
        post_damage_value_cents: u64,
    },
    CollateralLoan {
        loan_to_value_bps: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AppraisalRecord {
    item_description: String,
    purpose: AppraisalPurpose,
    appraised_value_cents: u64,
    appraiser_id: String,
    date_epoch_days: u32,
    photographs: Vec<String>,
    comparable_sales: Vec<u64>,
}

// --- Casting process states ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum CastingProcessState {
    WaxModelCreated {
        model_id: String,
    },
    WaxTreeAssembled {
        model_count: u16,
        sprue_diameter_um: u32,
    },
    InvestmentCoated {
        plaster_type: String,
        vacuum_degassed: bool,
    },
    Burnout {
        peak_temperature_c: u16,
        hold_time_minutes: u16,
    },
    MetalMelted {
        metal: String,
        temperature_c: u16,
        flux_used: Option<String>,
    },
    Poured {
        centrifugal: bool,
        vacuum_assist: bool,
    },
    Quenched {
        delay_seconds: u32,
        water_temperature_c: u16,
    },
    Devested,
    CutFromTree,
    Finished {
        polish_steps: Vec<String>,
        final_weight_mg: u32,
    },
}

// --- Laser inscription records ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct LaserInscription {
    inscription_text: String,
    location: String,
    depth_nanometers: u32,
    laser_type: String,
    magnification_required: u8,
    verification_image_hash: Option<String>,
}

// --- Jewelry piece aggregate ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct JewelryPiece {
    sku: String,
    description: String,
    metal: PreciousMetalAlloy,
    settings: Vec<SettingType>,
    hallmarks: Vec<HallmarkStamp>,
    weight_milligrams: u32,
}

// ===================== TEST FUNCTIONS =====================

// --- Test 1: Diamond grading with fancy vivid color ---
#[test]
fn test_diamond_grading_fancy_vivid_roundtrip() {
    let val = DiamondGrading {
        carat_weight_milligrams: 2150,
        cut: DiamondCut::Ideal,
        clarity: DiamondClarity::VVS1,
        color: DiamondColor::FancyVivid {
            hue: "Yellow".to_string(),
            saturation: 9,
        },
        fluorescence: Some("None".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode DiamondGrading fancy vivid");
    let (decoded, _): (DiamondGrading, usize) =
        decode_from_slice(&bytes).expect("decode DiamondGrading fancy vivid");
    assert_eq!(val, decoded);
}

// --- Test 2: Diamond grading colorless no fluorescence ---
#[test]
fn test_diamond_grading_colorless_roundtrip() {
    let val = DiamondGrading {
        carat_weight_milligrams: 503,
        cut: DiamondCut::Excellent,
        clarity: DiamondClarity::Flawless,
        color: DiamondColor::Colorless("D".to_string()),
        fluorescence: None,
    };
    let bytes = encode_to_vec(&val).expect("encode DiamondGrading colorless");
    let (decoded, _): (DiamondGrading, usize) =
        decode_from_slice(&bytes).expect("decode DiamondGrading colorless");
    assert_eq!(val, decoded);
}

// --- Test 3: Colored stone sapphire with origin ---
#[test]
fn test_colored_stone_sapphire_roundtrip() {
    let val = ColoredStoneGrading {
        stone_type: ColoredStoneType::Sapphire {
            color_modifier: "Padparadscha".to_string(),
        },
        carat_weight_milligrams: 4320,
        hue: "pinkish-orange".to_string(),
        tone: 5,
        saturation: 7,
        clarity_type: "Type II".to_string(),
        origin_country: Some("Sri Lanka".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode ColoredStoneGrading sapphire");
    let (decoded, _): (ColoredStoneGrading, usize) =
        decode_from_slice(&bytes).expect("decode ColoredStoneGrading sapphire");
    assert_eq!(val, decoded);
}

// --- Test 4: Opal with play of color ---
#[test]
fn test_colored_stone_opal_roundtrip() {
    let val = ColoredStoneGrading {
        stone_type: ColoredStoneType::Opal {
            play_of_color: true,
            body_tone: "black".to_string(),
        },
        carat_weight_milligrams: 6100,
        hue: "multicolor".to_string(),
        tone: 2,
        saturation: 8,
        clarity_type: "Transparent".to_string(),
        origin_country: Some("Australia".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode ColoredStoneGrading opal");
    let (decoded, _): (ColoredStoneGrading, usize) =
        decode_from_slice(&bytes).expect("decode ColoredStoneGrading opal");
    assert_eq!(val, decoded);
}

// --- Test 5: Gold alloy two-tone composition ---
#[test]
fn test_precious_metal_gold_alloy_roundtrip() {
    let val = PreciousMetalAlloy::Gold {
        karat: 18,
        color: "Rose".to_string(),
        alloy_composition_ppm: vec![750_000, 125_000, 125_000],
    };
    let bytes = encode_to_vec(&val).expect("encode PreciousMetalAlloy gold");
    let (decoded, _): (PreciousMetalAlloy, usize) =
        decode_from_slice(&bytes).expect("decode PreciousMetalAlloy gold");
    assert_eq!(val, decoded);
}

// --- Test 6: Platinum alloy with iridium ---
#[test]
fn test_precious_metal_platinum_roundtrip() {
    let val = PreciousMetalAlloy::Platinum {
        purity_ppt: 950,
        iridium_ppm: 50_000,
    };
    let bytes = encode_to_vec(&val).expect("encode PreciousMetalAlloy platinum");
    let (decoded, _): (PreciousMetalAlloy, usize) =
        decode_from_slice(&bytes).expect("decode PreciousMetalAlloy platinum");
    assert_eq!(val, decoded);
}

// --- Test 7: Invisible setting grid ---
#[test]
fn test_setting_invisible_grid_roundtrip() {
    let val = SettingType::Invisible {
        grid_rows: 4,
        grid_cols: 4,
    };
    let bytes = encode_to_vec(&val).expect("encode SettingType invisible");
    let (decoded, _): (SettingType, usize) =
        decode_from_slice(&bytes).expect("decode SettingType invisible");
    assert_eq!(val, decoded);
}

// --- Test 8: Micro-pave setting ---
#[test]
fn test_setting_micro_pave_roundtrip() {
    let val = SettingType::Pave {
        micro_pave: true,
        stone_diameter_um: 800,
    };
    let bytes = encode_to_vec(&val).expect("encode SettingType micro pave");
    let (decoded, _): (SettingType, usize) =
        decode_from_slice(&bytes).expect("decode SettingType micro pave");
    assert_eq!(val, decoded);
}

// --- Test 9: Hallmark stamps with assay office ---
#[test]
fn test_hallmark_assay_office_roundtrip() {
    let val = HallmarkStamp::AssayOfficeMark {
        office: "London".to_string(),
        year: 2025,
    };
    let bytes = encode_to_vec(&val).expect("encode HallmarkStamp assay office");
    let (decoded, _): (HallmarkStamp, usize) =
        decode_from_slice(&bytes).expect("decode HallmarkStamp assay office");
    assert_eq!(val, decoded);
}

// --- Test 10: Laser-engraved hallmark ---
#[test]
fn test_hallmark_laser_engraved_roundtrip() {
    let val = HallmarkStamp::LaserEngraved {
        text: "PT950".to_string(),
        font_size_um: 120,
    };
    let bytes = encode_to_vec(&val).expect("encode HallmarkStamp laser engraved");
    let (decoded, _): (HallmarkStamp, usize) =
        decode_from_slice(&bytes).expect("decode HallmarkStamp laser engraved");
    assert_eq!(val, decoded);
}

// --- Test 11: GIA certification report ---
#[test]
fn test_certification_gia_roundtrip() {
    let val = CertificationBody::GIA {
        report_number: 2_235_678_901,
        report_type: "Diamond Grading".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode CertificationBody GIA");
    let (decoded, _): (CertificationBody, usize) =
        decode_from_slice(&bytes).expect("decode CertificationBody GIA");
    assert_eq!(val, decoded);
}

// --- Test 12: SSEF certification with advanced testing ---
#[test]
fn test_certification_ssef_advanced_testing_roundtrip() {
    let val = CertificationBody::SSEF {
        advanced_testing: vec![
            "LIBS".to_string(),
            "Raman Spectroscopy".to_string(),
            "FTIR".to_string(),
            "UV-Vis-NIR".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode CertificationBody SSEF");
    let (decoded, _): (CertificationBody, usize) =
        decode_from_slice(&bytes).expect("decode CertificationBody SSEF");
    assert_eq!(val, decoded);
}

// --- Test 13: Repair work order stone replacement ---
#[test]
fn test_repair_stone_replacement_roundtrip() {
    let val = RepairWorkOrder::StoneReplacement {
        position: "center prong setting".to_string(),
        original_stone: "CZ 6mm round".to_string(),
        replacement_stone: "Moissanite 6mm round".to_string(),
        carat_milligrams: 850,
    };
    let bytes = encode_to_vec(&val).expect("encode RepairWorkOrder stone replacement");
    let (decoded, _): (RepairWorkOrder, usize) =
        decode_from_slice(&bytes).expect("decode RepairWorkOrder stone replacement");
    assert_eq!(val, decoded);
}

// --- Test 14: Repair work order laser welding ---
#[test]
fn test_repair_laser_welding_roundtrip() {
    let val = RepairWorkOrder::LaserWelding {
        weld_points: vec![(120, 340), (125, 345), (130, 350), (135, 355)],
        power_watts: 45,
    };
    let bytes = encode_to_vec(&val).expect("encode RepairWorkOrder laser welding");
    let (decoded, _): (RepairWorkOrder, usize) =
        decode_from_slice(&bytes).expect("decode RepairWorkOrder laser welding");
    assert_eq!(val, decoded);
}

// --- Test 15: CAD ring profile with comfort fit ---
#[test]
fn test_cad_ring_profile_roundtrip() {
    let val = CadDesignParameter::RingProfile {
        width_um: 4_500,
        thickness_um: 1_800,
        comfort_fit: true,
        profile_shape: "D-shape".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode CadDesignParameter ring profile");
    let (decoded, _): (CadDesignParameter, usize) =
        decode_from_slice(&bytes).expect("decode CadDesignParameter ring profile");
    assert_eq!(val, decoded);
}

// --- Test 16: CAD pendant with bezier path ---
#[test]
fn test_cad_bezier_path_roundtrip() {
    let val = CadDesignParameter::BezierPath {
        control_points: vec![(0, 0), (1000, 3000), (3000, 3000), (4000, 0), (2000, -1500)],
        resolution: 256,
    };
    let bytes = encode_to_vec(&val).expect("encode CadDesignParameter bezier path");
    let (decoded, _): (CadDesignParameter, usize) =
        decode_from_slice(&bytes).expect("decode CadDesignParameter bezier path");
    assert_eq!(val, decoded);
}

// --- Test 17: Natural stone origin with mine name ---
#[test]
fn test_stone_origin_natural_roundtrip() {
    let val = StoneOrigin::Natural {
        mine_name: Some("Mogok Valley".to_string()),
        country: "Myanmar".to_string(),
        region: Some("Mandalay".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode StoneOrigin natural");
    let (decoded, _): (StoneOrigin, usize) =
        decode_from_slice(&bytes).expect("decode StoneOrigin natural");
    assert_eq!(val, decoded);
}

// --- Test 18: Lab-grown origin with CVD method ---
#[test]
fn test_stone_origin_lab_grown_roundtrip() {
    let val = StoneOrigin::LabGrown {
        method: "CVD".to_string(),
        manufacturer: "Element Six".to_string(),
        seed_crystal: Some("Type IIa natural diamond".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode StoneOrigin lab grown");
    let (decoded, _): (StoneOrigin, usize) =
        decode_from_slice(&bytes).expect("decode StoneOrigin lab grown");
    assert_eq!(val, decoded);
}

// --- Test 19: Appraisal record insurance replacement ---
#[test]
fn test_appraisal_insurance_replacement_roundtrip() {
    let val = AppraisalRecord {
        item_description:
            "18K white gold engagement ring with 1.52ct round brilliant diamond, D/VVS1".to_string(),
        purpose: AppraisalPurpose::InsuranceReplacement,
        appraised_value_cents: 2_450_000,
        appraiser_id: "GJ-2025-4481".to_string(),
        date_epoch_days: 20165,
        photographs: vec![
            "front_view.sha256:ab12cd34".to_string(),
            "side_view.sha256:ef56gh78".to_string(),
            "macro_stone.sha256:ij90kl12".to_string(),
        ],
        comparable_sales: vec![2_300_000, 2_500_000, 2_380_000, 2_520_000],
    };
    let bytes = encode_to_vec(&val).expect("encode AppraisalRecord insurance");
    let (decoded, _): (AppraisalRecord, usize) =
        decode_from_slice(&bytes).expect("decode AppraisalRecord insurance");
    assert_eq!(val, decoded);
}

// --- Test 20: Casting process burnout and pour states ---
#[test]
fn test_casting_process_burnout_roundtrip() {
    let burnout = CastingProcessState::Burnout {
        peak_temperature_c: 732,
        hold_time_minutes: 120,
    };
    let poured = CastingProcessState::Poured {
        centrifugal: true,
        vacuum_assist: false,
    };
    let finished = CastingProcessState::Finished {
        polish_steps: vec![
            "220 grit".to_string(),
            "400 grit".to_string(),
            "pre-polish".to_string(),
            "rouge".to_string(),
        ],
        final_weight_mg: 4_820,
    };
    let states = vec![burnout, poured, finished];
    let bytes = encode_to_vec(&states).expect("encode casting process states");
    let (decoded, _): (Vec<CastingProcessState>, usize) =
        decode_from_slice(&bytes).expect("decode casting process states");
    assert_eq!(states, decoded);
}

// --- Test 21: Laser inscription record ---
#[test]
fn test_laser_inscription_roundtrip() {
    let val = LaserInscription {
        inscription_text: "GIA 2235678901".to_string(),
        location: "girdle".to_string(),
        depth_nanometers: 500,
        laser_type: "Nd:YAG 1064nm".to_string(),
        magnification_required: 10,
        verification_image_hash: Some(
            "sha256:3a7bd3e2360a3d29eea436fcfb7e44c735d117c42d1c1835420b6b9942dd4f1b".to_string(),
        ),
    };
    let bytes = encode_to_vec(&val).expect("encode LaserInscription");
    let (decoded, _): (LaserInscription, usize) =
        decode_from_slice(&bytes).expect("decode LaserInscription");
    assert_eq!(val, decoded);
}

// --- Test 22: Full jewelry piece aggregate with multiple settings and hallmarks ---
#[test]
fn test_jewelry_piece_full_aggregate_roundtrip() {
    let val = JewelryPiece {
        sku: "RNG-ENG-00471".to_string(),
        description: "Platinum solitaire engagement ring with cathedral arch setting".to_string(),
        metal: PreciousMetalAlloy::Platinum {
            purity_ppt: 950,
            iridium_ppm: 50_000,
        },
        settings: vec![
            SettingType::Prong {
                prong_count: 6,
                prong_shape: "rounded".to_string(),
            },
            SettingType::CathedralArch { height_mm: 8 },
            SettingType::Pave {
                micro_pave: true,
                stone_diameter_um: 900,
            },
        ],
        hallmarks: vec![
            HallmarkStamp::PurityMark {
                metal: "Platinum".to_string(),
                purity_value: "950".to_string(),
            },
            HallmarkStamp::MakersMark("KJEWEL".to_string()),
            HallmarkStamp::LaserEngraved {
                text: "GIA 2235678901".to_string(),
                font_size_um: 100,
            },
        ],
        weight_milligrams: 8_340,
    };
    let bytes = encode_to_vec(&val).expect("encode JewelryPiece full aggregate");
    let (decoded, _): (JewelryPiece, usize) =
        decode_from_slice(&bytes).expect("decode JewelryPiece full aggregate");
    assert_eq!(val, decoded);
}
