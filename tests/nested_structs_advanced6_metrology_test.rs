//! Metrology/yield/defect-focused tests for nested_structs_advanced6 (split from nested_structs_advanced6_test.rs).

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
// Domain types: Die coordinate (shared with wafer and etest files)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieCoordinate {
    row: u16,
    col: u16,
    site: u8,
}

// ---------------------------------------------------------------------------
// Metrology measurements
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CdSemMeasurement {
    feature_name: String,
    target_cd_nm: f64,
    measured_cd_nm: f64,
    lwr_nm: f64,
    lwe_nm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OcdMeasurement {
    profile_name: String,
    cd_top_nm: f64,
    cd_bottom_nm: f64,
    sidewall_angle_deg: f64,
    height_nm: f64,
    goodness_of_fit: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FilmThickness {
    film_type: String,
    target_nm: f64,
    measured_nm: f64,
    uniformity_pct: f64,
    measurement_points: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetrologyReport {
    wafer_id: String,
    step_name: String,
    cd_sem: Vec<CdSemMeasurement>,
    ocd: Vec<OcdMeasurement>,
    film: Vec<FilmThickness>,
    timestamp_epoch: u64,
}

// ---------------------------------------------------------------------------
// Yield analysis
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BinCode {
    bin_number: u16,
    bin_name: String,
    pass_fail: bool,
    count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieBinResult {
    die: DieCoordinate,
    hard_bin: u16,
    soft_bin: u16,
    test_time_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferMap {
    wafer_id: String,
    die_results: Vec<DieBinResult>,
    total_die: u32,
    good_die: u32,
    yield_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldReport {
    lot_id: String,
    wafer_maps: Vec<WaferMap>,
    bin_definitions: Vec<BinCode>,
    lot_yield_pct: f64,
}

// ---------------------------------------------------------------------------
// Clean room environment
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParticleCount {
    size_threshold_um: f64,
    count_per_cubic_ft: u32,
    measurement_location: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentReading {
    temperature_c: f64,
    humidity_pct: f64,
    pressure_pa: f64,
    airflow_m_per_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CleanRoomZone {
    zone_name: String,
    iso_class: u8,
    environment: EnvironmentReading,
    particle_counts: Vec<ParticleCount>,
    personnel_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FabEnvironment {
    fab_name: String,
    zones: Vec<CleanRoomZone>,
    total_area_sqm: f64,
    monitoring_interval_sec: u32,
}

// ---------------------------------------------------------------------------
// Defect classification
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectLocation {
    die: DieCoordinate,
    x_um: f64,
    y_um: f64,
    layer: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectFeature {
    size_um: f64,
    aspect_ratio: f64,
    brightness: f64,
    polarity: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectEntry {
    defect_id: u64,
    location: DefectLocation,
    feature: DefectFeature,
    classification: String,
    killer: bool,
    review_completed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefectInspectionReport {
    wafer_id: String,
    tool_id: String,
    recipe_name: String,
    defects: Vec<DefectEntry>,
    total_defects: u32,
    killer_defects: u32,
    defect_density_per_cm2: f64,
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_metrology_report_cd_sem() {
    let report = MetrologyReport {
        wafer_id: "W01-N7-0315".into(),
        step_name: "POST-ETCH-M1".into(),
        cd_sem: vec![
            CdSemMeasurement {
                feature_name: "M1_LINE_H".into(),
                target_cd_nm: 28.0,
                measured_cd_nm: 27.6,
                lwr_nm: 1.8,
                lwe_nm: 1.2,
            },
            CdSemMeasurement {
                feature_name: "M1_SPACE_V".into(),
                target_cd_nm: 28.0,
                measured_cd_nm: 28.3,
                lwr_nm: 2.0,
                lwe_nm: 1.5,
            },
        ],
        ocd: vec![OcdMeasurement {
            profile_name: "M1_TRENCH".into(),
            cd_top_nm: 29.5,
            cd_bottom_nm: 26.0,
            sidewall_angle_deg: 86.5,
            height_nm: 100.0,
            goodness_of_fit: 0.998,
        }],
        film: vec![],
        timestamp_epoch: 1710489600,
    };
    let encoded = encode_to_vec(&report).expect("encode metrology report");
    let (decoded, _): (MetrologyReport, _) =
        decode_from_slice(&encoded).expect("decode metrology report");
    assert_eq!(report, decoded);
}

#[test]
fn test_metrology_film_thickness() {
    let report = MetrologyReport {
        wafer_id: "W12-N5-0410".into(),
        step_name: "POST-DEP-ILD".into(),
        cd_sem: vec![],
        ocd: vec![],
        film: vec![
            FilmThickness {
                film_type: "SiO2-PECVD".into(),
                target_nm: 500.0,
                measured_nm: 498.3,
                uniformity_pct: 1.2,
                measurement_points: 49,
            },
            FilmThickness {
                film_type: "SiN-Cap".into(),
                target_nm: 30.0,
                measured_nm: 30.8,
                uniformity_pct: 2.1,
                measurement_points: 49,
            },
        ],
        timestamp_epoch: 1710576000,
    };
    let encoded = encode_to_vec(&report).expect("encode film report");
    let (decoded, _): (MetrologyReport, _) =
        decode_from_slice(&encoded).expect("decode film report");
    assert_eq!(report, decoded);
}

#[test]
fn test_yield_analysis_wafer_map() {
    let yield_rpt = YieldReport {
        lot_id: "LOT-2026-0315-B".into(),
        wafer_maps: vec![WaferMap {
            wafer_id: "W03".into(),
            die_results: vec![
                DieBinResult {
                    die: DieCoordinate {
                        row: 5,
                        col: 10,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 450,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 5,
                        col: 11,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 460,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 6,
                        col: 10,
                        site: 1,
                    },
                    hard_bin: 5,
                    soft_bin: 510,
                    test_time_ms: 380,
                },
            ],
            total_die: 500,
            good_die: 480,
            yield_pct: 96.0,
        }],
        bin_definitions: vec![
            BinCode {
                bin_number: 1,
                bin_name: "PASS".into(),
                pass_fail: true,
                count: 480,
            },
            BinCode {
                bin_number: 5,
                bin_name: "FAIL-LEAK".into(),
                pass_fail: false,
                count: 20,
            },
        ],
        lot_yield_pct: 96.0,
    };
    let encoded = encode_to_vec(&yield_rpt).expect("encode yield report");
    let (decoded, _): (YieldReport, _) = decode_from_slice(&encoded).expect("decode yield report");
    assert_eq!(yield_rpt, decoded);
}

#[test]
fn test_clean_room_environment() {
    let env = FabEnvironment {
        fab_name: "FAB-12-HSINCHU".into(),
        zones: vec![
            CleanRoomZone {
                zone_name: "LITHO-BAY".into(),
                iso_class: 3,
                environment: EnvironmentReading {
                    temperature_c: 22.0,
                    humidity_pct: 43.0,
                    pressure_pa: 101425.0,
                    airflow_m_per_s: 0.45,
                },
                particle_counts: vec![
                    ParticleCount {
                        size_threshold_um: 0.1,
                        count_per_cubic_ft: 35,
                        measurement_location: "Above-Scanner".into(),
                    },
                    ParticleCount {
                        size_threshold_um: 0.5,
                        count_per_cubic_ft: 2,
                        measurement_location: "Track-Load".into(),
                    },
                ],
                personnel_count: 3,
            },
            CleanRoomZone {
                zone_name: "DIFFUSION-BAY".into(),
                iso_class: 5,
                environment: EnvironmentReading {
                    temperature_c: 22.5,
                    humidity_pct: 44.0,
                    pressure_pa: 101400.0,
                    airflow_m_per_s: 0.35,
                },
                particle_counts: vec![ParticleCount {
                    size_threshold_um: 0.5,
                    count_per_cubic_ft: 80,
                    measurement_location: "Furnace-Front".into(),
                }],
                personnel_count: 2,
            },
        ],
        total_area_sqm: 12000.0,
        monitoring_interval_sec: 300,
    };
    let encoded = encode_to_vec(&env).expect("encode fab environment");
    let (decoded, _): (FabEnvironment, _) =
        decode_from_slice(&encoded).expect("decode fab environment");
    assert_eq!(env, decoded);
}

#[test]
fn test_defect_inspection_report() {
    let report = DefectInspectionReport {
        wafer_id: "W19-N7-INSP".into(),
        tool_id: "KLA-2925-01".into(),
        recipe_name: "M1-POST-ETCH-BRIGHT".into(),
        defects: vec![
            DefectEntry {
                defect_id: 10001,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 8,
                        col: 12,
                        site: 1,
                    },
                    x_um: 1523.4,
                    y_um: 2801.7,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.08,
                    aspect_ratio: 1.2,
                    brightness: 180.0,
                    polarity: "BRIGHT".into(),
                },
                classification: "BRIDGE".into(),
                killer: true,
                review_completed: true,
            },
            DefectEntry {
                defect_id: 10002,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 9,
                        col: 14,
                        site: 1,
                    },
                    x_um: 3100.5,
                    y_um: 450.2,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.15,
                    aspect_ratio: 3.5,
                    brightness: 50.0,
                    polarity: "DARK".into(),
                },
                classification: "OPEN".into(),
                killer: true,
                review_completed: true,
            },
            DefectEntry {
                defect_id: 10003,
                location: DefectLocation {
                    die: DieCoordinate {
                        row: 3,
                        col: 5,
                        site: 1,
                    },
                    x_um: 800.0,
                    y_um: 1200.0,
                    layer: "METAL1".into(),
                },
                feature: DefectFeature {
                    size_um: 0.04,
                    aspect_ratio: 1.0,
                    brightness: 120.0,
                    polarity: "BRIGHT".into(),
                },
                classification: "PARTICLE".into(),
                killer: false,
                review_completed: false,
            },
        ],
        total_defects: 3,
        killer_defects: 2,
        defect_density_per_cm2: 0.15,
    };
    let encoded = encode_to_vec(&report).expect("encode defect report");
    let (decoded, _): (DefectInspectionReport, _) =
        decode_from_slice(&encoded).expect("decode defect report");
    assert_eq!(report, decoded);
}

#[test]
fn test_multi_wafer_yield_report() {
    let make_map = |id: &str, good: u32, total: u32| -> WaferMap {
        WaferMap {
            wafer_id: id.into(),
            die_results: vec![
                DieBinResult {
                    die: DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    hard_bin: 1,
                    soft_bin: 100,
                    test_time_ms: 500,
                },
                DieBinResult {
                    die: DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 1,
                    },
                    hard_bin: 2,
                    soft_bin: 200,
                    test_time_ms: 520,
                },
            ],
            total_die: total,
            good_die: good,
            yield_pct: (good as f64 / total as f64) * 100.0,
        }
    };

    let report = YieldReport {
        lot_id: "LOT-MULTI-YIELD".into(),
        wafer_maps: vec![
            make_map("W01", 490, 500),
            make_map("W02", 485, 500),
            make_map("W03", 495, 500),
        ],
        bin_definitions: vec![
            BinCode {
                bin_number: 1,
                bin_name: "PASS".into(),
                pass_fail: true,
                count: 1470,
            },
            BinCode {
                bin_number: 2,
                bin_name: "FAIL-VT".into(),
                pass_fail: false,
                count: 30,
            },
        ],
        lot_yield_pct: 98.0,
    };
    let encoded = encode_to_vec(&report).expect("encode multi-wafer yield");
    let (decoded, _): (YieldReport, _) =
        decode_from_slice(&encoded).expect("decode multi-wafer yield");
    assert_eq!(report, decoded);
}

#[test]
fn test_empty_defect_report() {
    let report = DefectInspectionReport {
        wafer_id: "W25-CLEAN".into(),
        tool_id: "KLA-2925-02".into(),
        recipe_name: "BARE-SI-BASELINE".into(),
        defects: vec![],
        total_defects: 0,
        killer_defects: 0,
        defect_density_per_cm2: 0.0,
    };
    let encoded = encode_to_vec(&report).expect("encode clean defect report");
    let (decoded, _): (DefectInspectionReport, _) =
        decode_from_slice(&encoded).expect("decode clean defect report");
    assert_eq!(report, decoded);
}
