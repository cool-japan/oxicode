//! Wafer/litho/etch-focused tests for nested_structs_advanced6 (split from nested_structs_advanced6_test.rs).

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
// Domain types: Wafer lot tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieCoordinate {
    row: u16,
    col: u16,
    site: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferIdentifier {
    wafer_number: u8,
    slot: u8,
    orientation_notch_deg: f32,
    die_grid: Vec<DieCoordinate>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LotInfo {
    lot_id: String,
    product_code: String,
    technology_node_nm: u16,
    wafer_count: u8,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaferLot {
    lot: LotInfo,
    wafers: Vec<WaferIdentifier>,
    route_id: String,
    current_step: u32,
    total_steps: u32,
}

// ---------------------------------------------------------------------------
// Photolithography parameters
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OverlayError {
    x_offset_nm: f64,
    y_offset_nm: f64,
    rotation_urad: f64,
    magnification_ppm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExposureSettings {
    dose_mj_per_cm2: f64,
    focus_offset_nm: f64,
    numerical_aperture: f64,
    sigma_inner: f64,
    sigma_outer: f64,
    wavelength_nm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReticleInfo {
    reticle_id: String,
    layer_name: String,
    field_size_x_mm: f64,
    field_size_y_mm: f64,
    pellicle_attached: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LithoStep {
    step_name: String,
    tool_id: String,
    reticle: ReticleInfo,
    exposure: ExposureSettings,
    overlay: OverlayError,
    alignment_marks_used: u8,
}

// ---------------------------------------------------------------------------
// Etch process recipes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GasFlow {
    gas_name: String,
    flow_sccm: f64,
    mfc_channel: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RfSource {
    frequency_mhz: f64,
    power_watts: f64,
    pulsed: bool,
    duty_cycle_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtchStep {
    step_label: String,
    duration_sec: f64,
    pressure_mtorr: f64,
    temperature_c: f64,
    gases: Vec<GasFlow>,
    source_rf: RfSource,
    bias_rf: RfSource,
    endpoint_signal: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtchRecipe {
    recipe_id: String,
    chamber_id: String,
    steps: Vec<EtchStep>,
    total_time_sec: f64,
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_wafer_lot_tracking() {
    let lot = WaferLot {
        lot: LotInfo {
            lot_id: "N7-2026-0315-A".into(),
            product_code: "APEX7".into(),
            technology_node_nm: 7,
            wafer_count: 25,
            priority: 1,
        },
        wafers: vec![
            WaferIdentifier {
                wafer_number: 1,
                slot: 1,
                orientation_notch_deg: 0.0,
                die_grid: vec![
                    DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 1,
                        col: 0,
                        site: 1,
                    },
                ],
            },
            WaferIdentifier {
                wafer_number: 2,
                slot: 2,
                orientation_notch_deg: 0.0,
                die_grid: vec![
                    DieCoordinate {
                        row: 0,
                        col: 0,
                        site: 1,
                    },
                    DieCoordinate {
                        row: 0,
                        col: 1,
                        site: 2,
                    },
                ],
            },
        ],
        route_id: "ROUTE-N7-LOGIC-V3".into(),
        current_step: 42,
        total_steps: 320,
    };
    let encoded = encode_to_vec(&lot).expect("encode wafer lot");
    let (decoded, _): (WaferLot, _) = decode_from_slice(&encoded).expect("decode wafer lot");
    assert_eq!(lot, decoded);
}

#[test]
fn test_photolithography_full_step() {
    let litho = LithoStep {
        step_name: "M1_PHOTO".into(),
        tool_id: "ASML-NXT2000-01".into(),
        reticle: ReticleInfo {
            reticle_id: "RTL-M1-V2R3".into(),
            layer_name: "METAL1".into(),
            field_size_x_mm: 26.0,
            field_size_y_mm: 33.0,
            pellicle_attached: true,
        },
        exposure: ExposureSettings {
            dose_mj_per_cm2: 30.5,
            focus_offset_nm: -10.0,
            numerical_aperture: 0.33,
            sigma_inner: 0.6,
            sigma_outer: 0.9,
            wavelength_nm: 13.5,
        },
        overlay: OverlayError {
            x_offset_nm: 0.8,
            y_offset_nm: -0.3,
            rotation_urad: 0.12,
            magnification_ppm: 0.05,
        },
        alignment_marks_used: 8,
    };
    let encoded = encode_to_vec(&litho).expect("encode litho step");
    let (decoded, _): (LithoStep, _) = decode_from_slice(&encoded).expect("decode litho step");
    assert_eq!(litho, decoded);
}

#[test]
fn test_etch_recipe_multi_step() {
    let recipe = EtchRecipe {
        recipe_id: "ETCH-POLY-HK-V4".into(),
        chamber_id: "LAM-KIYO-C3".into(),
        steps: vec![
            EtchStep {
                step_label: "Breakthrough".into(),
                duration_sec: 5.0,
                pressure_mtorr: 4.0,
                temperature_c: 60.0,
                gases: vec![
                    GasFlow {
                        gas_name: "Cl2".into(),
                        flow_sccm: 100.0,
                        mfc_channel: 1,
                    },
                    GasFlow {
                        gas_name: "BCl3".into(),
                        flow_sccm: 50.0,
                        mfc_channel: 2,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 800.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 50.0,
                    pulsed: true,
                    duty_cycle_pct: 30.0,
                },
                endpoint_signal: None,
            },
            EtchStep {
                step_label: "Main Etch".into(),
                duration_sec: 45.0,
                pressure_mtorr: 8.0,
                temperature_c: 55.0,
                gases: vec![
                    GasFlow {
                        gas_name: "HBr".into(),
                        flow_sccm: 200.0,
                        mfc_channel: 3,
                    },
                    GasFlow {
                        gas_name: "O2".into(),
                        flow_sccm: 5.0,
                        mfc_channel: 4,
                    },
                    GasFlow {
                        gas_name: "He".into(),
                        flow_sccm: 300.0,
                        mfc_channel: 5,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 400.0,
                    pulsed: true,
                    duty_cycle_pct: 50.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 120.0,
                    pulsed: true,
                    duty_cycle_pct: 40.0,
                },
                endpoint_signal: Some("OES-Br-827nm".into()),
            },
            EtchStep {
                step_label: "Over Etch".into(),
                duration_sec: 15.0,
                pressure_mtorr: 12.0,
                temperature_c: 55.0,
                gases: vec![
                    GasFlow {
                        gas_name: "HBr".into(),
                        flow_sccm: 150.0,
                        mfc_channel: 3,
                    },
                    GasFlow {
                        gas_name: "O2".into(),
                        flow_sccm: 10.0,
                        mfc_channel: 4,
                    },
                ],
                source_rf: RfSource {
                    frequency_mhz: 60.0,
                    power_watts: 200.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                bias_rf: RfSource {
                    frequency_mhz: 13.56,
                    power_watts: 30.0,
                    pulsed: false,
                    duty_cycle_pct: 100.0,
                },
                endpoint_signal: None,
            },
        ],
        total_time_sec: 65.0,
    };
    let encoded = encode_to_vec(&recipe).expect("encode etch recipe");
    let (decoded, _): (EtchRecipe, _) = decode_from_slice(&encoded).expect("decode etch recipe");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_overlay_tight_tolerance() {
    let litho = LithoStep {
        step_name: "VIA1_PHOTO".into(),
        tool_id: "ASML-NXE3400-02".into(),
        reticle: ReticleInfo {
            reticle_id: "RTL-V1-V1R1".into(),
            layer_name: "VIA1".into(),
            field_size_x_mm: 26.0,
            field_size_y_mm: 33.0,
            pellicle_attached: true,
        },
        exposure: ExposureSettings {
            dose_mj_per_cm2: 33.0,
            focus_offset_nm: -5.0,
            numerical_aperture: 0.33,
            sigma_inner: 0.5,
            sigma_outer: 0.85,
            wavelength_nm: 13.5,
        },
        overlay: OverlayError {
            x_offset_nm: 0.15,
            y_offset_nm: -0.08,
            rotation_urad: 0.02,
            magnification_ppm: 0.01,
        },
        alignment_marks_used: 16,
    };
    let encoded = encode_to_vec(&litho).expect("encode overlay litho");
    let (decoded, _): (LithoStep, _) = decode_from_slice(&encoded).expect("decode overlay litho");
    assert_eq!(litho, decoded);
}
