#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types for brewery and distillery production management ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MashTunReading {
    batch_id: String,
    timestamp_epoch: u64,
    temperature_celsius: f64,
    ph_level: f64,
    volume_liters: f64,
    grain_bed_depth_cm: f64,
    recirculation_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FermentationStage {
    Pitching {
        yeast_strain: String,
        cell_count_billions: u64,
    },
    Lag {
        hours_elapsed: u32,
    },
    Active {
        gravity_reading: f64,
        co2_volumes: f64,
        temperature_celsius: f64,
    },
    Conditioning {
        days_elapsed: u32,
        diacetyl_rest: bool,
    },
    ColdCrash {
        target_temp_celsius: f64,
        duration_hours: u32,
    },
    Complete {
        final_gravity: f64,
        abv_percent: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct YeastStrain {
    strain_code: String,
    species: String,
    lab_origin: String,
    attenuation_low: f64,
    attenuation_high: f64,
    temperature_range_low_c: f64,
    temperature_range_high_c: f64,
    flocculation: String,
    alcohol_tolerance_percent: f64,
    phenolic: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HopVariety {
    name: String,
    origin_country: String,
    alpha_acid_percent: f64,
    beta_acid_percent: f64,
    cohumulone_percent: f64,
    total_oil_ml_per_100g: f64,
    myrcene_percent: f64,
    humulene_percent: f64,
    caryophyllene_percent: f64,
    aroma_descriptors: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct IbuCalculation {
    hop_name: String,
    weight_grams: f64,
    alpha_acid_percent: f64,
    boil_time_minutes: u32,
    wort_volume_liters: f64,
    wort_gravity: f64,
    utilization_factor: f64,
    calculated_ibu: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BarrelAgingEntry {
    barrel_id: String,
    barrel_type: String,
    previous_contents: String,
    char_level: u8,
    capacity_liters: f64,
    fill_date_epoch: u64,
    target_aging_days: u32,
    warehouse_location: String,
    temperature_log: Vec<f64>,
    humidity_log: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DistillationColumnReading {
    still_id: String,
    column_plates: u8,
    feed_rate_liters_per_hour: f64,
    reflux_ratio: f64,
    head_temperature_celsius: f64,
    base_temperature_celsius: f64,
    condenser_water_temp_celsius: f64,
    vapor_speed_m_per_s: f64,
    abv_at_collection: f64,
    cut_type: DistillationCut,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DistillationCut {
    Foreshots,
    Heads,
    Hearts,
    Tails,
    Feints,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpiritProofCalc {
    spirit_name: String,
    measured_abv: f64,
    temperature_celsius: f64,
    corrected_abv: f64,
    proof_us: f64,
    proof_uk: f64,
    volume_liters: f64,
    liters_of_pure_alcohol: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GrainBill {
    recipe_id: String,
    total_weight_kg: f64,
    grains: Vec<GrainAddition>,
    expected_og: f64,
    mash_efficiency_target: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GrainAddition {
    grain_name: String,
    weight_kg: f64,
    percentage: f64,
    color_lovibond: f64,
    ppg: f64,
    requires_milling: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WaterChemistryProfile {
    source_name: String,
    calcium_ppm: f64,
    magnesium_ppm: f64,
    sodium_ppm: f64,
    sulfate_ppm: f64,
    chloride_ppm: f64,
    bicarbonate_ppm: f64,
    ph: f64,
    total_hardness_ppm: f64,
    residual_alkalinity: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CaskStrengthMeasurement {
    cask_id: String,
    distillery: String,
    spirit_type: String,
    age_years: u32,
    abv_percent: f64,
    volume_remaining_liters: f64,
    angel_share_percent: f64,
    tasting_notes: Vec<String>,
    color_descriptor: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BlendingRatio {
    blend_name: String,
    master_blender: String,
    target_abv: f64,
    components: Vec<BlendComponent>,
    batch_volume_liters: f64,
    approved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BlendComponent {
    cask_id: String,
    spirit_type: String,
    age_years: u32,
    proportion_percent: f64,
    abv_percent: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BottlingLineTelemetry {
    line_id: String,
    timestamp_epoch: u64,
    bottles_per_minute: f64,
    fill_volume_ml: f64,
    fill_deviation_ml: f64,
    cap_torque_nm: f64,
    label_alignment_offset_mm: f64,
    reject_count: u32,
    total_filled: u64,
    status: BottlingLineStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BottlingLineStatus {
    Running,
    Paused {
        reason: String,
    },
    Changeover {
        next_product: String,
        est_minutes: u32,
    },
    Maintenance {
        work_order: String,
    },
    Shutdown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QualityControlSample {
    sample_id: String,
    batch_id: String,
    analyst: String,
    abv_measured: f64,
    color_srm: f64,
    turbidity_ntu: f64,
    dissolved_o2_ppb: f64,
    co2_volumes: f64,
    microbiological_pass: bool,
    sensory_score: f64,
    verdict: QcVerdict,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum QcVerdict {
    Pass,
    ConditionalPass { notes: String },
    Fail { reason: String },
    PendingReview,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BrewhouseSchedule {
    brew_date_epoch: u64,
    recipe_name: String,
    brewer: String,
    mash_in_time_epoch: u64,
    lauter_start_epoch: u64,
    boil_start_epoch: u64,
    whirlpool_epoch: u64,
    knockout_epoch: u64,
    target_fermenter: String,
    estimated_volume_liters: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FermenterPressureLog {
    fermenter_id: String,
    batch_id: String,
    readings: Vec<PressureReading>,
    spunding_valve_set_psi: f64,
    max_rated_psi: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PressureReading {
    timestamp_epoch: u64,
    pressure_psi: f64,
    temperature_celsius: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WhiskyMaturationRecord {
    distillery_name: String,
    cask_id: String,
    spirit_type: String,
    distillation_date_epoch: u64,
    cask_type: String,
    warehouse_id: String,
    rack_position: String,
    fill_abv: f64,
    current_abv: f64,
    fill_volume_liters: f64,
    current_volume_liters: f64,
    samples_taken: Vec<MaturationSample>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MaturationSample {
    date_epoch: u64,
    abv_percent: f64,
    color_descriptor: String,
    nose_notes: Vec<String>,
    palate_notes: Vec<String>,
    finish_notes: Vec<String>,
    score: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WortBoilAddition {
    ingredient: String,
    addition_type: BoilAdditionType,
    weight_grams: f64,
    time_minutes: u32,
    purpose: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BoilAdditionType {
    Bittering,
    Flavor,
    Aroma,
    Whirlpool,
    DryHop,
    Fining,
    Nutrient,
    Adjunct,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CleanInPlaceLog {
    equipment_id: String,
    cip_cycle_id: String,
    steps: Vec<CipStep>,
    total_water_liters: f64,
    chemical_cost_cents: u64,
    operator: String,
    verified: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CipStep {
    step_name: String,
    chemical: Option<String>,
    concentration_percent: Option<f64>,
    temperature_celsius: f64,
    duration_minutes: u32,
    flow_rate_lpm: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TaxComplianceRecord {
    period_start_epoch: u64,
    period_end_epoch: u64,
    liters_produced: f64,
    liters_bottled: f64,
    liters_in_bond: f64,
    liters_duty_paid: f64,
    duty_rate_per_liter: f64,
    total_duty_owed_cents: u64,
    excise_number: String,
    bond_warehouse_id: String,
}

// --- Tests ---

#[test]
fn test_mash_tun_reading_roundtrip() {
    let val = MashTunReading {
        batch_id: "BREW-2026-0315-A".to_string(),
        timestamp_epoch: 1773724800,
        temperature_celsius: 66.5,
        ph_level: 5.35,
        volume_liters: 1200.0,
        grain_bed_depth_cm: 42.0,
        recirculation_active: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode mash tun reading");
    let (decoded, _): (MashTunReading, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode mash tun reading");
    assert_eq!(val, decoded);
}

#[test]
fn test_fermentation_stages_all_variants() {
    let stages = vec![
        FermentationStage::Pitching {
            yeast_strain: "WLP001".to_string(),
            cell_count_billions: 200,
        },
        FermentationStage::Lag { hours_elapsed: 8 },
        FermentationStage::Active {
            gravity_reading: 1.045,
            co2_volumes: 1.2,
            temperature_celsius: 19.0,
        },
        FermentationStage::Conditioning {
            days_elapsed: 5,
            diacetyl_rest: true,
        },
        FermentationStage::ColdCrash {
            target_temp_celsius: 1.0,
            duration_hours: 48,
        },
        FermentationStage::Complete {
            final_gravity: 1.010,
            abv_percent: 5.2,
        },
    ];
    for stage in &stages {
        let bytes = encode_to_vec(stage, config::standard()).expect("encode fermentation stage");
        let (decoded, _): (FermentationStage, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode fermentation stage");
        assert_eq!(*stage, decoded);
    }
}

#[test]
fn test_yeast_strain_properties() {
    let val = YeastStrain {
        strain_code: "WY1056".to_string(),
        species: "Saccharomyces cerevisiae".to_string(),
        lab_origin: "Wyeast".to_string(),
        attenuation_low: 73.0,
        attenuation_high: 77.0,
        temperature_range_low_c: 15.0,
        temperature_range_high_c: 22.0,
        flocculation: "Low-Medium".to_string(),
        alcohol_tolerance_percent: 11.0,
        phenolic: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode yeast strain");
    let (decoded, _): (YeastStrain, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode yeast strain");
    assert_eq!(val, decoded);
}

#[test]
fn test_hop_variety_with_aroma_descriptors() {
    let val = HopVariety {
        name: "Citra".to_string(),
        origin_country: "USA".to_string(),
        alpha_acid_percent: 12.0,
        beta_acid_percent: 4.0,
        cohumulone_percent: 22.0,
        total_oil_ml_per_100g: 2.5,
        myrcene_percent: 60.0,
        humulene_percent: 11.0,
        caryophyllene_percent: 6.0,
        aroma_descriptors: vec![
            "Citrus".to_string(),
            "Tropical".to_string(),
            "Grapefruit".to_string(),
            "Passion Fruit".to_string(),
            "Lychee".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode hop variety");
    let (decoded, _): (HopVariety, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode hop variety");
    assert_eq!(val, decoded);
    assert_eq!(decoded.aroma_descriptors.len(), 5);
}

#[test]
fn test_ibu_calculation_tinseth_method() {
    let val = IbuCalculation {
        hop_name: "Magnum".to_string(),
        weight_grams: 28.0,
        alpha_acid_percent: 14.0,
        boil_time_minutes: 60,
        wort_volume_liters: 23.0,
        wort_gravity: 1.055,
        utilization_factor: 0.241,
        calculated_ibu: 40.7,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode ibu calculation");
    let (decoded, _): (IbuCalculation, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode ibu calculation");
    assert_eq!(val, decoded);
}

#[test]
fn test_barrel_aging_with_temperature_logs() {
    let val = BarrelAgingEntry {
        barrel_id: "BRN-2024-0042".to_string(),
        barrel_type: "American White Oak".to_string(),
        previous_contents: "Bourbon".to_string(),
        char_level: 3,
        capacity_liters: 200.0,
        fill_date_epoch: 1704067200,
        target_aging_days: 365,
        warehouse_location: "RH-A-03-12".to_string(),
        temperature_log: vec![14.2, 15.1, 18.3, 22.7, 25.1, 23.8, 19.5, 15.0, 12.8, 13.1],
        humidity_log: vec![72.0, 70.5, 68.0, 65.0, 62.0, 64.0, 69.0, 73.0, 75.0, 74.0],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode barrel aging entry");
    let (decoded, _): (BarrelAgingEntry, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode barrel aging entry");
    assert_eq!(val, decoded);
    assert_eq!(decoded.temperature_log.len(), 10);
    assert_eq!(decoded.humidity_log.len(), 10);
}

#[test]
fn test_distillation_column_reading_hearts_cut() {
    let val = DistillationColumnReading {
        still_id: "POT-STILL-01".to_string(),
        column_plates: 0,
        feed_rate_liters_per_hour: 150.0,
        reflux_ratio: 0.0,
        head_temperature_celsius: 78.3,
        base_temperature_celsius: 98.1,
        condenser_water_temp_celsius: 12.0,
        vapor_speed_m_per_s: 0.8,
        abv_at_collection: 72.5,
        cut_type: DistillationCut::Hearts,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode distillation reading");
    let (decoded, _): (DistillationColumnReading, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode distillation reading");
    assert_eq!(val, decoded);

    // Also verify all cut variants roundtrip
    let cuts = vec![
        DistillationCut::Foreshots,
        DistillationCut::Heads,
        DistillationCut::Hearts,
        DistillationCut::Tails,
        DistillationCut::Feints,
    ];
    for cut in &cuts {
        let cut_bytes = encode_to_vec(cut, config::standard()).expect("encode distillation cut");
        let (cut_decoded, _): (DistillationCut, usize) =
            decode_owned_from_slice(&cut_bytes, config::standard())
                .expect("decode distillation cut");
        assert_eq!(*cut, cut_decoded);
    }
}

#[test]
fn test_spirit_proof_calculation() {
    let val = SpiritProofCalc {
        spirit_name: "Single Malt Scotch".to_string(),
        measured_abv: 63.5,
        temperature_celsius: 20.0,
        corrected_abv: 63.5,
        proof_us: 127.0,
        proof_uk: 111.1,
        volume_liters: 190.0,
        liters_of_pure_alcohol: 120.65,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode spirit proof calc");
    let (decoded, _): (SpiritProofCalc, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode spirit proof calc");
    assert_eq!(val, decoded);
}

#[test]
fn test_grain_bill_with_multiple_additions() {
    let val = GrainBill {
        recipe_id: "IPA-WEST-007".to_string(),
        total_weight_kg: 6.8,
        grains: vec![
            GrainAddition {
                grain_name: "Maris Otter Pale Malt".to_string(),
                weight_kg: 5.5,
                percentage: 80.9,
                color_lovibond: 3.0,
                ppg: 38.0,
                requires_milling: true,
            },
            GrainAddition {
                grain_name: "Crystal 60L".to_string(),
                weight_kg: 0.45,
                percentage: 6.6,
                color_lovibond: 60.0,
                ppg: 34.0,
                requires_milling: true,
            },
            GrainAddition {
                grain_name: "Munich Malt".to_string(),
                weight_kg: 0.55,
                percentage: 8.1,
                color_lovibond: 10.0,
                ppg: 37.0,
                requires_milling: true,
            },
            GrainAddition {
                grain_name: "Flaked Oats".to_string(),
                weight_kg: 0.3,
                percentage: 4.4,
                color_lovibond: 1.0,
                ppg: 33.0,
                requires_milling: false,
            },
        ],
        expected_og: 1.065,
        mash_efficiency_target: 75.0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode grain bill");
    let (decoded, _): (GrainBill, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode grain bill");
    assert_eq!(val, decoded);
    assert_eq!(decoded.grains.len(), 4);
}

#[test]
fn test_water_chemistry_profile_burton_style() {
    let val = WaterChemistryProfile {
        source_name: "Burton-on-Trent (target)".to_string(),
        calcium_ppm: 275.0,
        magnesium_ppm: 40.0,
        sodium_ppm: 25.0,
        sulfate_ppm: 610.0,
        chloride_ppm: 35.0,
        bicarbonate_ppm: 260.0,
        ph: 7.8,
        total_hardness_ppm: 850.0,
        residual_alkalinity: 45.0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode water chemistry");
    let (decoded, _): (WaterChemistryProfile, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode water chemistry");
    assert_eq!(val, decoded);
}

#[test]
fn test_cask_strength_measurement() {
    let val = CaskStrengthMeasurement {
        cask_id: "SPEY-1998-0127".to_string(),
        distillery: "Speyside Distillery".to_string(),
        spirit_type: "Single Malt Scotch Whisky".to_string(),
        age_years: 28,
        abv_percent: 52.3,
        volume_remaining_liters: 155.0,
        angel_share_percent: 22.5,
        tasting_notes: vec![
            "Dried fruit".to_string(),
            "Sherry".to_string(),
            "Dark chocolate".to_string(),
            "Leather".to_string(),
            "Old oak".to_string(),
        ],
        color_descriptor: "Deep amber".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode cask strength");
    let (decoded, _): (CaskStrengthMeasurement, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode cask strength");
    assert_eq!(val, decoded);
}

#[test]
fn test_blending_ratio_multi_component() {
    let val = BlendingRatio {
        blend_name: "Master Reserve 18".to_string(),
        master_blender: "Takeshi Yamamoto".to_string(),
        target_abv: 43.0,
        components: vec![
            BlendComponent {
                cask_id: "SHR-2006-0044".to_string(),
                spirit_type: "Sherry Cask Malt".to_string(),
                age_years: 20,
                proportion_percent: 35.0,
                abv_percent: 55.2,
            },
            BlendComponent {
                cask_id: "BRB-2007-0112".to_string(),
                spirit_type: "Bourbon Cask Malt".to_string(),
                age_years: 19,
                proportion_percent: 40.0,
                abv_percent: 58.1,
            },
            BlendComponent {
                cask_id: "MZN-2005-0008".to_string(),
                spirit_type: "Mizunara Oak Malt".to_string(),
                age_years: 21,
                proportion_percent: 25.0,
                abv_percent: 51.8,
            },
        ],
        batch_volume_liters: 2400.0,
        approved: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode blending ratio");
    let (decoded, _): (BlendingRatio, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode blending ratio");
    assert_eq!(val, decoded);
    assert_eq!(decoded.components.len(), 3);
    let total_pct: f64 = decoded
        .components
        .iter()
        .map(|c| c.proportion_percent)
        .sum();
    assert!((total_pct - 100.0).abs() < 0.001);
}

#[test]
fn test_bottling_line_telemetry_running() {
    let val = BottlingLineTelemetry {
        line_id: "BOTTLING-LINE-03".to_string(),
        timestamp_epoch: 1773724800,
        bottles_per_minute: 120.5,
        fill_volume_ml: 700.0,
        fill_deviation_ml: 0.3,
        cap_torque_nm: 1.8,
        label_alignment_offset_mm: 0.1,
        reject_count: 4,
        total_filled: 14460,
        status: BottlingLineStatus::Running,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode bottling telemetry");
    let (decoded, _): (BottlingLineTelemetry, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode bottling telemetry");
    assert_eq!(val, decoded);
}

#[test]
fn test_bottling_line_status_variants() {
    let statuses = vec![
        BottlingLineStatus::Running,
        BottlingLineStatus::Paused {
            reason: "Label roll changeover".to_string(),
        },
        BottlingLineStatus::Changeover {
            next_product: "12yr Single Malt".to_string(),
            est_minutes: 45,
        },
        BottlingLineStatus::Maintenance {
            work_order: "WO-2026-1183".to_string(),
        },
        BottlingLineStatus::Shutdown,
    ];
    for status in &statuses {
        let bytes = encode_to_vec(status, config::standard()).expect("encode bottling status");
        let (decoded, _): (BottlingLineStatus, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode bottling status");
        assert_eq!(*status, decoded);
    }
}

#[test]
fn test_quality_control_sample_pass() {
    let val = QualityControlSample {
        sample_id: "QC-2026-03-0042".to_string(),
        batch_id: "BREW-2026-0310-B".to_string(),
        analyst: "Kenji Watanabe".to_string(),
        abv_measured: 5.15,
        color_srm: 8.2,
        turbidity_ntu: 0.45,
        dissolved_o2_ppb: 28.0,
        co2_volumes: 2.45,
        microbiological_pass: true,
        sensory_score: 8.5,
        verdict: QcVerdict::Pass,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode qc sample");
    let (decoded, _): (QualityControlSample, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode qc sample");
    assert_eq!(val, decoded);
}

#[test]
fn test_qc_verdict_conditional_and_fail() {
    let verdicts = vec![
        QcVerdict::Pass,
        QcVerdict::ConditionalPass {
            notes: "Slight haze; acceptable for unfiltered style".to_string(),
        },
        QcVerdict::Fail {
            reason: "Diacetyl above threshold at 45 ppb".to_string(),
        },
        QcVerdict::PendingReview,
    ];
    for verdict in &verdicts {
        let bytes = encode_to_vec(verdict, config::standard()).expect("encode qc verdict");
        let (decoded, _): (QcVerdict, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode qc verdict");
        assert_eq!(*verdict, decoded);
    }
}

#[test]
fn test_brewhouse_schedule_full_day() {
    let val = BrewhouseSchedule {
        brew_date_epoch: 1773724800,
        recipe_name: "Hazy Juicy IPA".to_string(),
        brewer: "Sakura Tanaka".to_string(),
        mash_in_time_epoch: 1773746400,
        lauter_start_epoch: 1773750000,
        boil_start_epoch: 1773753600,
        whirlpool_epoch: 1773757200,
        knockout_epoch: 1773758400,
        target_fermenter: "FV-07".to_string(),
        estimated_volume_liters: 2000.0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode brewhouse schedule");
    let (decoded, _): (BrewhouseSchedule, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode brewhouse schedule");
    assert_eq!(val, decoded);
    assert!(decoded.lauter_start_epoch > decoded.mash_in_time_epoch);
    assert!(decoded.boil_start_epoch > decoded.lauter_start_epoch);
}

#[test]
fn test_fermenter_pressure_log_multiple_readings() {
    let val = FermenterPressureLog {
        fermenter_id: "FV-12".to_string(),
        batch_id: "BREW-2026-0312-C".to_string(),
        readings: vec![
            PressureReading {
                timestamp_epoch: 1773724800,
                pressure_psi: 0.5,
                temperature_celsius: 18.0,
            },
            PressureReading {
                timestamp_epoch: 1773728400,
                pressure_psi: 2.1,
                temperature_celsius: 18.5,
            },
            PressureReading {
                timestamp_epoch: 1773732000,
                pressure_psi: 5.8,
                temperature_celsius: 19.2,
            },
            PressureReading {
                timestamp_epoch: 1773735600,
                pressure_psi: 9.3,
                temperature_celsius: 19.8,
            },
            PressureReading {
                timestamp_epoch: 1773739200,
                pressure_psi: 12.0,
                temperature_celsius: 20.0,
            },
            PressureReading {
                timestamp_epoch: 1773742800,
                pressure_psi: 14.5,
                temperature_celsius: 20.1,
            },
        ],
        spunding_valve_set_psi: 15.0,
        max_rated_psi: 30.0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode pressure log");
    let (decoded, _): (FermenterPressureLog, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode pressure log");
    assert_eq!(val, decoded);
    assert_eq!(decoded.readings.len(), 6);
    for reading in &decoded.readings {
        assert!(reading.pressure_psi <= decoded.max_rated_psi);
    }
}

#[test]
fn test_whisky_maturation_record_with_samples() {
    let val = WhiskyMaturationRecord {
        distillery_name: "Highland Spring Distillery".to_string(),
        cask_id: "HSD-2014-0233".to_string(),
        spirit_type: "Single Malt".to_string(),
        distillation_date_epoch: 1393632000,
        cask_type: "First Fill Oloroso Sherry Butt".to_string(),
        warehouse_id: "WH-03".to_string(),
        rack_position: "R12-T4".to_string(),
        fill_abv: 69.5,
        current_abv: 56.2,
        fill_volume_liters: 500.0,
        current_volume_liters: 410.0,
        samples_taken: vec![
            MaturationSample {
                date_epoch: 1551398400,
                abv_percent: 64.1,
                color_descriptor: "Light gold".to_string(),
                nose_notes: vec!["Vanilla".to_string(), "New make".to_string()],
                palate_notes: vec!["Sweet".to_string(), "Malty".to_string()],
                finish_notes: vec!["Short".to_string(), "Clean".to_string()],
                score: 60,
            },
            MaturationSample {
                date_epoch: 1677628800,
                abv_percent: 59.8,
                color_descriptor: "Amber".to_string(),
                nose_notes: vec![
                    "Dried fruit".to_string(),
                    "Toffee".to_string(),
                    "Oak".to_string(),
                ],
                palate_notes: vec![
                    "Rich".to_string(),
                    "Spice".to_string(),
                    "Chocolate".to_string(),
                ],
                finish_notes: vec!["Medium-long".to_string(), "Warming".to_string()],
                score: 82,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode maturation record");
    let (decoded, _): (WhiskyMaturationRecord, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode maturation record");
    assert_eq!(val, decoded);
    assert_eq!(decoded.samples_taken.len(), 2);
    assert!(decoded.current_abv < decoded.fill_abv);
    assert!(decoded.current_volume_liters < decoded.fill_volume_liters);
}

#[test]
fn test_wort_boil_additions_full_schedule() {
    let additions = vec![
        WortBoilAddition {
            ingredient: "Magnum".to_string(),
            addition_type: BoilAdditionType::Bittering,
            weight_grams: 30.0,
            time_minutes: 60,
            purpose: "Base bitterness to 35 IBU".to_string(),
        },
        WortBoilAddition {
            ingredient: "Mosaic".to_string(),
            addition_type: BoilAdditionType::Flavor,
            weight_grams: 25.0,
            time_minutes: 15,
            purpose: "Berry and stone fruit flavor".to_string(),
        },
        WortBoilAddition {
            ingredient: "Citra".to_string(),
            addition_type: BoilAdditionType::Whirlpool,
            weight_grams: 50.0,
            time_minutes: 0,
            purpose: "Tropical aroma at flameout".to_string(),
        },
        WortBoilAddition {
            ingredient: "Whirlfloc".to_string(),
            addition_type: BoilAdditionType::Fining,
            weight_grams: 5.0,
            time_minutes: 10,
            purpose: "Protein coagulation for clarity".to_string(),
        },
        WortBoilAddition {
            ingredient: "Yeast nutrient".to_string(),
            addition_type: BoilAdditionType::Nutrient,
            weight_grams: 3.0,
            time_minutes: 10,
            purpose: "Healthy fermentation support".to_string(),
        },
    ];
    let bytes = encode_to_vec(&additions, config::standard()).expect("encode boil additions");
    let (decoded, _): (Vec<WortBoilAddition>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode boil additions");
    assert_eq!(additions, decoded);
    assert_eq!(decoded.len(), 5);
}

#[test]
fn test_clean_in_place_log_multi_step() {
    let val = CleanInPlaceLog {
        equipment_id: "FV-07".to_string(),
        cip_cycle_id: "CIP-2026-0315-002".to_string(),
        steps: vec![
            CipStep {
                step_name: "Pre-rinse".to_string(),
                chemical: None,
                concentration_percent: None,
                temperature_celsius: 45.0,
                duration_minutes: 5,
                flow_rate_lpm: 120.0,
            },
            CipStep {
                step_name: "Caustic wash".to_string(),
                chemical: Some("Sodium hydroxide".to_string()),
                concentration_percent: Some(2.0),
                temperature_celsius: 80.0,
                duration_minutes: 20,
                flow_rate_lpm: 100.0,
            },
            CipStep {
                step_name: "Intermediate rinse".to_string(),
                chemical: None,
                concentration_percent: None,
                temperature_celsius: 50.0,
                duration_minutes: 5,
                flow_rate_lpm: 120.0,
            },
            CipStep {
                step_name: "Acid wash".to_string(),
                chemical: Some("Peracetic acid".to_string()),
                concentration_percent: Some(0.5),
                temperature_celsius: 25.0,
                duration_minutes: 15,
                flow_rate_lpm: 100.0,
            },
            CipStep {
                step_name: "Final rinse".to_string(),
                chemical: None,
                concentration_percent: None,
                temperature_celsius: 20.0,
                duration_minutes: 5,
                flow_rate_lpm: 120.0,
            },
        ],
        total_water_liters: 2500.0,
        chemical_cost_cents: 1250,
        operator: "Hiroshi Sato".to_string(),
        verified: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode cip log");
    let (decoded, _): (CleanInPlaceLog, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode cip log");
    assert_eq!(val, decoded);
    assert_eq!(decoded.steps.len(), 5);
    assert!(decoded.verified);
}

#[test]
fn test_tax_compliance_record_duty_calculation() {
    let val = TaxComplianceRecord {
        period_start_epoch: 1772524800,
        period_end_epoch: 1773724800,
        liters_produced: 15000.0,
        liters_bottled: 12500.0,
        liters_in_bond: 85000.0,
        liters_duty_paid: 12500.0,
        duty_rate_per_liter: 28.74,
        total_duty_owed_cents: 35925000,
        excise_number: "EX-JP-2026-00412".to_string(),
        bond_warehouse_id: "BOND-WH-07".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode tax compliance");
    let (decoded, _): (TaxComplianceRecord, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode tax compliance");
    assert_eq!(val, decoded);
    assert!(decoded.liters_bottled <= decoded.liters_produced);
}
