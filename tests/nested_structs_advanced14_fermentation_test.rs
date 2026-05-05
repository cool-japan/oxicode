//! Fermentation, barrel, and blending-focused tests for nested_structs_advanced14 (split from nested_structs_advanced14_test.rs).

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
// Domain types — Grape & Harvest (needed for BlendComponent)
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
// Helpers
// ---------------------------------------------------------------------------

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

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {}", ctx));
    let (decoded, consumed): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {}", ctx));
    assert_eq!(val, &decoded, "roundtrip mismatch for {}", ctx);
    assert_eq!(consumed, bytes.len(), "byte count mismatch for {}", ctx);
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
