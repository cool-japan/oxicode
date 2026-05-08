//! Lab, tasting, and bottling-focused tests for nested_structs_advanced14 (split from nested_structs_advanced14_test.rs).

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
// Helpers
// ---------------------------------------------------------------------------

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

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {}", ctx));
    let (decoded, consumed): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {}", ctx));
    assert_eq!(val, &decoded, "roundtrip mismatch for {}", ctx);
    assert_eq!(consumed, bytes.len(), "byte count mismatch for {}", ctx);
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
